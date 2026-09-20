# Engineering Specification & Implementation Blueprint: TermLoom
> Modern Cross-Platform Terminal Session Recorder & Vector Animator (Next-Gen termtosvg)

---

## 1. Executive Summary

### 1.1 The Opportunity
`termtosvg` (11.5k GitHub stars) was one of the most widely appreciated developer tools for recording terminal sessions to vector SVG animations. However, it has been unmaintained since 2020 and suffers from critical shortcomings:
* **Outdated Runtime:** Built for legacy Python 3.5; broken dependency chains on Python 3.10+.
* **Zero Native Windows Support:** Relies on Unix pseudo-terminals (`pty`); completely non-functional on native Windows (PowerShell / CMD) without WSL.
* **Legacy Escape Sequence Support:** Lacks support for TrueColor (24-bit RGB), modern Nerd Font glyphs, emojis, and OSC 8 hyperlinks.
* **Bloated Outputs:** Lacks frame deduplication and idle time compression, resulting in multi-megabyte SVGs for brief terminal sessions.

### 1.2 The Solution: TermLoom
**TermLoom** is a modern, single-binary, cross-platform terminal recorder and vector animator written in **Rust**. It provides zero-dependency execution across Windows, macOS, and Linux, capturing raw terminal streams via native platform PTYs (Windows ConPTY and POSIX openpty) and compiling them into:
1. **Pure Vector Animated SVGs:** Zero-JS, CSS-keyframe animated SVGs with native `@media (prefers-color-scheme)` dark/light theme switching.
2. **Interactive Standalone HTML Players:** Lightweight single-file players with timeline scrubbers, speed controls, and a dedicated 1-click **"Copy Command"** button.
3. **Asciinema v2 (`.cast`) Interoperability:** Seamless import/export compatibility with the wider terminal recording ecosystem.

---

## 2. Technical Stack & Dependencies

* **Language:** Rust (2021 edition) - for memory safety, deterministic performance, cross-compilation, and single-binary distribution.
* **Core Crates:**
  * `portable-pty` (`^0.8`): Unified cross-platform PTY abstraction supporting Windows ConPTY and POSIX `openpty`.
  * `vte` (`^0.13`) / `alacritty_terminal`: Fast, standard-compliant ANSI / VT100 parser and terminal grid state machine.
  * `clap` (`^4.5`, derive): Declarative, type-safe CLI argument parsing.
  * `unicode-width` (`^0.1`): Exact terminal column calculation for multi-byte Unicode, emojis, and East Asian wide characters.
  * `serde` & `serde_json` (`^1.0`): Asciinema v2 `.cast` JSONL serialization and deserialization.
  * `xxhash-rust` (`^0.8`): Ultra-fast 64-bit hashing for screen grid frame deduplication.
  * `minijinja` (`^2.0`): Lightweight, dependency-free template engine for compiling SVG and HTML artifacts.

---

## 3. High-Level Architecture

```
┌────────────────────────────────────────────────────────────────────────┐
│                          TermLoom Architecture                         │
└────────────────────────────────────────────────────────────────────────┘

[ Interactive Shell Process ] (powershell.exe / zsh / bash)
              │
              ▼
[ PTY Engine: portable-pty ]
  ├── Windows: Native ConPTY (Console Pseudo-Console API)
  └── Unix/macOS: POSIX openpty (termios master/slave)
              │  (Raw byte stream with ANSI/OSC codes)
              ▼
[ Virtual Terminal Emulator (VTE Grid) ]
  ├── Screen Grid Matrix: rows x cols
  ├── Cell State: Grapheme cluster, 24-bit FG/BG color, SGR flags
  └── Cursor State: (X, Y) coordinates, blink rate, visibility
              │  (Snapshots captured on output buffer mutations)
              ▼
[ Frame Pipeline & Compression Engine ]
  ├── xxHash-based Frame Deduplication (discards visually identical frames)
  ├── Idle Time Clamping (--max-wait: truncates long pauses to e.g. 1.2s)
  └── Trailing Silence Trimmer (strips trailing wait before shell exit)
              │
              ├─────────────────────────┼──────────────────────────┐
              ▼                         ▼                          ▼
   [ Animated SVG Exporter ]   [ Interactive HTML ]    [ Asciinema v2 (.cast) ]
     - Pure CSS keyframes        - Single-file bundle    - JSONL record format
     - prefers-color-scheme      - Scrub / Play / Pause  - Ecosystem interop
     - Retina-sharp vector       - "Copy Command" button
```

---

## 4. Component Design Specifications

### 4.1 Cross-Platform PTY Layer (`src/pty/`)
* **Process Spawning:** Auto-detects the host shell (`pwsh.exe` or `powershell.exe` on Windows, `$SHELL` or `/bin/bash` on Unix).
* **Terminal Dimensions:** Defaults to `cols: 100`, `rows: 28` (customizable via CLI flags).
* **Raw Mode:** Configures the host terminal into raw mode during recording to capture interactive keystrokes, Ctrl+C, tab completion, and escapes without interference.
* **Windows ConPTY:** Leverages `portable_pty::native_pty_system()` to bind to `CreatePseudoConsole`.

### 4.2 Terminal Grid & ANSI Parser (`src/emulator/`)
* **Cell Representation:**
  ```rust
  #[derive(Clone, PartialEq, Eq, Hash)]
  pub struct Cell {
      pub grapheme: String,
      pub width: usize,
      pub fg: Color, // 24-bit RGB, 256-palette, or ANSI 16
      pub bg: Color,
      pub bold: bool,
      pub italic: bool,
      pub underline: bool,
      pub inverse: bool,
      pub link: Option<String>, // OSC 8 hyperlinks
  }
  ```
* **Grid State:** Matrix of `Vec<Vec<Cell>>`. Handles cursor movements, scrollback, full clear (`\x1b[2J`), line clear (`\x1b[2K`), and cursor-relative operations.
* **Hyperlink Handling:** Parsed OSC 8 sequences are preserved and converted into clickable `<a xlink:href="...">` nodes in SVG and HTML outputs.

### 4.3 Frame Deduplication & Idle Time Clamping (`src/recorder/`)
* **Hashing:** Calculates an `xxhash64` hash of the active screen matrix. If a subsequent frame has the exact same visual hash as the previous one, it is merged into the preceding duration.
* **Idle Clamping:** When the interval between two terminal output bursts exceeds `--max-wait` (default: `1.5s`), it is automatically clamped to `--max-wait`.
* **Lead/Trail Trimming:** Trims any leading silence before the first keystroke and strips the final wait between shell termination and file saving.

### 4.4 Animated SVG Exporter (`src/exporters/svg/`)
* **Zero JavaScript:** Fully self-contained SVG using embedded CSS keyframe animations (`@keyframes step-animation`).
* **Optimized Rendering:**
  * Adjacent cells sharing identical foreground color and text styles are merged into single `<tspan>` tags.
  * Reusable colors are extracted into root CSS custom properties (`--fg-default`, `--bg-default`, etc.).
* **Responsive & Adaptive:**
  * Uses `@media (prefers-color-scheme: dark)` and `@media (prefers-color-scheme: light)` to adapt colors seamlessly according to user system settings.
* **Window Chrome:** Configurable window frame (macOS traffic lights, modern squircle border, or naked/headless).

### 4.5 Interactive HTML Player (`src/exporters/html/`)
* **Self-Contained:** Zero external CDNs or JavaScript dependencies; compiles into a single `.html` file.
* **Playback Controls:** Play, pause, timeline slider, speed presets (`0.5x`, `1x`, `1.5x`, `2x`), and fullscreen toggle.
* **Smart Command Extractor:** Analyzes prompt lines (matching `$` or `PS >`) to extract raw executed commands, presenting an instant **"Copy Command"** button for README readers.

---

## 5. CLI Command-Line Specification

```bash
# Record a session directly to an animated SVG
termloom record output.svg

# Record with custom geometry, custom theme, and max wait cap
termloom record --cols 110 --rows 30 --theme catppuccin-mocha --max-wait 1.2 demo.svg

# Record an explicit command without interactive shell
termloom record -c "git status && git log -n 5" git_demo.svg

# Record to an interactive standalone HTML player
termloom record --format html player.html

# Record to standard Asciinema v2 format (.cast)
termloom record --format cast session.cast

# Convert an existing .cast file into an optimized SVG
termloom convert session.cast -o output.svg --theme dracula --window-style macos
```

### CLI Options Matrix
| Flag | Description | Default |
| :--- | :--- | :--- |
| `-c, --command <CMD>` | Command to execute instead of default interactive shell | Default Shell |
| `--cols <COLS>` | Terminal columns | `100` |
| `--rows <ROWS>` | Terminal rows | `28` |
| `--max-wait <SECS>` | Maximum allowed idle time between frames | `1.5` |
| `--theme <NAME>` | Color theme (`catppuccin`, `dracula`, `nord`, `tokyo-night`, `gruvbox`) | `catppuccin-mocha` |
| `--window-style` | Window frame style (`macos`, `squircle`, `none`) | `macos` |
| `--format` | Output format (`svg`, `html`, `cast`) | Inferred from filename |

---

## 6. Project Repository Layout

```
termloom/
├── Cargo.toml
├── README.md
├── LICENSE-MIT
├── LICENSE-APACHE
├── assets/
│   ├── termloom_logo.svg
│   └── termloom_demo.svg
├── src/
│   ├── main.rs                   # Entry point and CLI dispatcher
│   ├── cli.rs                    # Clap derive models and validation
│   ├── pty/                      # Cross-platform PTY abstraction
│   │   ├── mod.rs
│   │   ├── session.rs            # Raw PTY session spawner
│   │   └── raw_mode.rs           # Host terminal raw mode guard
│   ├── emulator/                 # Virtual Terminal & ANSI Parser
│   │   ├── mod.rs
│   │   ├── cell.rs               # Cell, Style, and Color types
│   │   ├── grid.rs               # ScreenGrid matrix implementation
│   │   └── handler.rs            # VTE callback handler implementation
│   ├── recorder/                 # Frame Capture & Compression
│   │   ├── mod.rs
│   │   ├── event.rs              # Timestamped raw event
│   │   ├── frame.rs              # Snapshot frame representation
│   │   ├── compressor.rs         # Deduplication & idle clamping engine
│   │   └── cast.rs               # Asciinema v2 format parser/writer
│   ├── exporters/                # Output Renderers
│   │   ├── mod.rs
│   │   ├── svg/                  # SVG generator
│   │   │   ├── mod.rs
│   │   │   ├── template.rs       # Minijinja SVG template & CSS keyframes
│   │   │   └── window.rs         # Window decorations & traffic lights
│   │   └── html/                 # HTML Player generator
│   │       ├── mod.rs
│   │       └── player.html       # Single-file HTML/CSS/JS template
│   └── themes/                   # Color Palette Registry
│       ├── mod.rs
│       └── palette.rs            # 16-color ANSI & background mapping
└── tests/
    ├── vte_ansi_test.rs          # 24-bit TrueColor and SGR escape tests
    ├── deduplication_test.rs     # Frame hash compression assertions
    └── svg_output_test.rs        # Valid XML and viewBox assertions
```

---

## 7. Step-by-Step Implementation Roadmap

### Phase 1: Project Initialization & PTY Engine
1. Scaffold Rust project with `cargo new --bin termloom`.
2. Populate `Cargo.toml` with `portable-pty`, `clap`, `vte`, `serde`, `serde_json`, `unicode-width`, `xxhash-rust`, and `minijinja`.
3. Implement `pty::session`:
   * Spawn child process inside `portable-pty`.
   * Implement bidirectional I/O loop: forward user stdin to PTY while copying PTY stdout to user terminal and recording raw chunks with timestamps.
   * Ensure host terminal raw mode restores cleanly on exit or panic.

### Phase 2: Terminal Emulator & State Machine
1. Define `emulator::cell::Cell` with grapheme strings, 24-bit RGB colors, and SGR flags.
2. Implement `emulator::grid::ScreenGrid`:
   * Matrix storage `Vec<Vec<Cell>>` with `cols` and `rows`.
   * Cursor navigation, viewport scrolling, line clearing (`\x1b[2K`), and screen resets (`\x1b[2J`).
3. Connect `vte::Parser` callbacks to mutate `ScreenGrid` in response to ANSI and OSC 8 sequences.

### Phase 3: Frame Deduplication & Idle Clamping
1. Implement frame capturing at output boundaries.
2. In `recorder::compressor`:
   * Compute `xxhash64` of current screen grid; discard identical consecutive frames.
   * Apply idle time clamp (`--max-wait`): compress gaps > 1.5s down to 1.5s.
   * Strip initial startup lag and final shell exit delay.
3. Add `.cast` format import and export support.

### Phase 4: Modern Animated SVG Exporter
1. Build `exporters::svg`:
   * Calculate total animation duration `T_total`.
   * Compute CSS keyframe percentage offsets for each frame.
   * Generate SVG elements: window chrome, title bar, traffic light buttons, and `<text>` line containers.
   * Optimize tspan grouping: merge consecutive identical characters into unified blocks.
   * Inject dark/light theme switching via `@media (prefers-color-scheme: dark)`.

### Phase 5: Interactive HTML Player & CLI Polish
1. Implement `exporters::html`:
   * Create self-contained `player.html` with vanilla JS canvas/DOM renderer and SVG timeline controls.
   * Add prompt parser for automatic "Copy Command" button generation.
2. Add CLI progress feedback, recording stats (frames recorded, compression ratio, file size), and built-in themes.

### Phase 6: Advanced Automation, Security & Visual Enhancements (Completed)
1. **Scripted Automation Tape Runner (`termloom run <file.tape>`)**:
   * Lexer and AST parser for `.tape` commands (`Output`, `Set`, `Type`, `Enter`, `Space`, `Backspace`, `Sleep`, `Ctrl+C/D/L`).
   * Headless PTY execution driving shell inputs deterministically for CI/CD workflows.
2. **Credential & Secret Redaction Filter (`--mask-secrets`, `--redact-regex`)**:
   * Built-in patterns for GitHub PATs, AWS keys, Slack tokens, Bearer authorization tokens, and private keys.
   * In-place `ScreenGrid` cell character masking (`*`) preserving exact column geometry and color attributes.
3. **Instant Terminal Snapshots (`termloom snapshot`)**:
   * Single-frame vector SVG screenshot generation directly from commands (`-c`) or static terminal grids.
4. **CSS Hover-to-Pause & Custom Typography (`--hover-pause`, `--font-url`, `--shadow`)**:
   * `@media (hover: hover) { svg:hover .frame { animation-play-state: paused !important; } }`.
   * Dynamic external web font stylesheet injection via `@import url(...)`.
   * High-fidelity window drop-shadow filter styling.
5. **Session Feedback & Exit Trimming**:
   * Live window title recording status (`🔴 [REC] TermLoom (MM:SS)`).
   * Trailing `exit` command input stripping (`--trim-exit`).
   * Performance and compression report summary table.

---

## 8. Verification & Test Plan

### Automated Verification
* Run unit tests: `cargo test`
* **VTE Parser Test:** Validate cursor positioning, TrueColor 24-bit RGB codes, and emoji display widths.
* **Deduplication Test:** Assert that a 10-second idle terminal session collapses to a single frame.
* **SVG Validity:** Verify that exported `.svg` files pass XML schema parsing and render cleanly without layout errors.

### Manual Verification (Windows & Linux)
1. Run: `cargo run -- record --cols 100 --rows 28 test_session.svg`
2. Run interactive commands with syntax highlighting and emojis (`git status`, `ls -la`, `echo "⚡ Hello TermLoom"`).
3. Open `test_session.svg` in Chrome, Firefox, and VS Code SVG preview.
4. Verify smooth vector animation, sharp typography on high-DPI displays, and proper dark/light theme adaptation.
