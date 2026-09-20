# TermLoom Architecture

TermLoom translates terminal stream I/O into deterministic, lightweight vector animations. This document explains the data pipeline, subsystem responsibilities, and performance design.

## Subsystem Pipeline

The pipeline processes input through five stages:

```
[ Interactive Process / Command ]
              │
              ▼
    [ PTY Engine (portable-pty) ]
      Windows: ConPTY (CreatePseudoConsole)
      Unix/macOS: POSIX openpty
              │  (Raw byte stream with ANSI/OSC escapes)
              ▼
    [ Virtual Terminal Emulator (VTE) ]
      - Paul Flo Williams state machine (vte crate)
      - ScreenGrid 2D matrix (cols x rows)
      - Cell styling: 24-bit RGB, ANSI 256, SGR flags, OSC 8 links
      - Multi-column Unicode width resolution
              │  (Screen snapshots captured at read intervals)
              ▼
    [ Frame Compressor & Deduplicator ]
      - xxHash64 visual hashing per frame
      - Identical consecutive frame merging
      - Idle interval clamping (--max-wait)
      - Startup and shutdown latency trimming
              │
              ├─────────────────────────┼──────────────────────────┐
              ▼                         ▼                          ▼
    [ SVG Exporter ]           [ HTML Player ]            [ Asciinema v2 ]
      - Zero JS                  - Standalone bundle        - JSONL records
      - CSS @keyframes           - Timeline scrubbing       - Interop
      - tspan clustering         - Command extractor
      - prefers-color-scheme     - Copy button
```

## 1. Cross-Platform PTY Layer

Terminal sessions require an operating system pseudo-terminal to allocate a character device with dimension awareness, raw line discipline, and signal handling.

* **Windows**: TermLoom binds to the ConPTY API (`CreatePseudoConsole`). Windows pseudo-consoles translate Windows Console API calls made by processes like `powershell.exe` or `cmd.exe` into standard ANSI escape sequences.
* **Unix and macOS**: TermLoom allocates a master/slave pair using `openpty`, configures the slave terminal with termios flags, and attaches stdin/stdout file descriptors.

During interactive sessions, TermLoom places the host terminal into raw mode using `crossterm::terminal::enable_raw_mode`. A dedicated raw mode guard struct implements `Drop` to ensure terminal echo and line buffering are restored even if the application encounters a panic or `SIGINT`.

## 2. Virtual Terminal State Machine

Raw PTY bytes contain escape sequences that change cursor positions, clear screen regions, and apply color styles. TermLoom runs a virtual terminal emulator state machine to maintain an accurate in-memory representation of what the user saw on screen.

### Cell and ScreenGrid Model

The screen is represented by a `ScreenGrid`:

* `cols` and `rows`: Terminal dimensions.
* `cells`: A 2D array of `Cell` structs.
* `cursor`: Coordinate `(col, row)` and visibility flag.

Each `Cell` tracks:
* `grapheme`: A UTF-8 string representing the displayed character.
* `width`: Display width in columns (0 for zero-width combining characters, 1 for standard characters, 2 for wide Asian characters or emojis).
* `fg` and `bg`: Colors represented as 24-bit RGB (`Color::Rgb(r, g, b)`), 256-color indexes, or default palette references.
* `bold`, `italic`, `underline`, `inverse`, `strikethrough`: SGR boolean attributes.
* `link`: Optional target URL captured from OSC 8 escape sequences (`\x1b]8;;<url>\x1b\`).

### Unicode Column Calculations

Fixed-width terminal fonts allocate uniform cells for alphanumeric characters, but emojis and East Asian characters occupy two cell widths. TermLoom uses `unicode-width` to determine character display widths. When a double-width character is written at column `x`, the cell at `x + 1` is flagged as a wide continuation cell so text rendering does not cause coordinate drift.

## 3. Frame Deduplication and Idle Clamping

Naive screen recording captures hundreds of identical frames when the user pauses between typing, resulting in oversized animation files. TermLoom uses a two-stage compression strategy.

### Visual Hashing

Whenever new output arrives from the PTY, the emulator renders a snapshot. TermLoom computes a 64-bit hash of the entire grid state using xxHash (`xxh64`). If the visual hash matches the previous frame, no new frame is appended. Instead, the timestamp delta is added to the duration of the current frame.

### Idle Time Clamping

Terminal users often pause for multiple seconds while reading or thinking. The `--max-wait` flag (default: 1.5 seconds) caps the duration of any single frame. If a frame has an idle duration of 8 seconds, TermLoom truncates it to 1.5 seconds in the output timeline, keeping animations concise.

### Edge Trimming

* **Leading delay**: Pauses between session start and the first user keystroke are removed.
* **Trailing delay**: The pause between typing `exit` and the shell process terminating is trimmed.

## 4. Vector Animated SVG Exporter

The SVG exporter produces self-contained vector files without embedded JavaScript.

### CSS Keyframe Timing

TermLoom computes the total duration `T_total` of all compressed frames. For each frame `i` spanning time interval `[t_start, t_end]`:
* `p_start = (t_start / T_total) * 100%`
* `p_end = (t_end / T_total) * 100%`

Each frame is wrapped in an SVG `<g>` element whose visibility is governed by a CSS keyframe animation:

```css
@keyframes frame_i {
  0% { visibility: hidden; }
  p_start% { visibility: visible; }
  p_end% { visibility: hidden; }
  100% { visibility: hidden; }
}
```

### Text Run Optimization

Writing individual SVG `<text>` elements for every character creates large files. TermLoom clusters adjacent cells on the same line that share identical foreground colors, background colors, and text styles into continuous `<tspan>` runs.

### Dark and Light Theme Switching

Colors are declared using CSS custom properties. TermLoom injects `@media (prefers-color-scheme: dark)` and `@media (prefers-color-scheme: light)` rules so that users with light desktop themes see matching background and syntax colors.

## 5. Interactive HTML Player

For web documentation and READMEs needing interactive controls, TermLoom compiles a single self-contained HTML file.

* **Embedded canvas / DOM renderer**: Renders frames onto an HTML layout.
* **Controls**: Play/pause button, draggable timeline slider, elapsed/total time display, and speed multiplier selector (0.5x, 1x, 1.5x, 2x).
* **Command extraction**: Scans initial prompt lines for standard prompt patterns (`$`, `#`, `>`, `PS>`). Commands typed by the user are extracted and surfaced via a one-click "Copy Command" button.
* **Offline self-containment**: The HTML file contains all required CSS and JS inline without external CDN requests.
