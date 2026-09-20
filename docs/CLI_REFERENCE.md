# TermLoom CLI Reference

This document provides a detailed description of all command-line options and usage patterns available in TermLoom.

## Global Synopsis

```bash
termloom [COMMAND] [OPTIONS]
```

To view help or version information:

```bash
termloom --help
termloom --version
```

---

## Commands

### 1. `termloom record`

Records a terminal session to an output file.

```bash
termloom record [OPTIONS] [OUTPUT]
```

#### Arguments

* `[OUTPUT]`: The target file path. If omitted, defaults to `output.svg`. The output format is automatically inferred from the file extension (`.svg`, `.html`, `.cast`), unless explicitly overridden with `--format`.

#### Options

* `-c, --command <CMD>`  
  Runs the specified command directly inside the PTY instead of launching an interactive shell. Once the command exits, recording finishes automatically.  
  *Example*: `termloom record -c "ls -la" listing.svg`

* `--cols <COLS>`  
  Number of terminal columns.  
  *Default*: `100`

* `--rows <ROWS>`  
  Number of terminal rows.  
  *Default*: `28`

* `--theme <THEME>`  
  Name of the color palette to apply.  
  *Default*: `catppuccin-mocha`  
  *Available*: `catppuccin-mocha`, `catppuccin-latte`, `dracula`, `nord`, `tokyo-night`, `gruvbox-dark`, `solarized-dark`, `monokai`, `one-dark`.

* `--window-style <STYLE>`  
  Style of the window chrome frame rendered in SVG outputs.  
  *Default*: `macos`  
  *Available*:  
  * `macos`: Standard traffic-light window header with close, minimize, and zoom controls.  
  * `squircle`: Minimal rounded border without window buttons.  
  * `none`: Raw terminal grid without any window decoration.

* `--max-wait <SECS>`  
  Maximum idle duration permitted for any single frame in seconds. Frame intervals exceeding this threshold are clamped to this value.  
  *Default*: `1.5`

* `--format <FORMAT>`  
  Forces the output format regardless of the file extension.  
  *Available*: `svg`, `html`, `cast`.

* `--title <TITLE>`  
  Window title text displayed in the header bar.  
  *Default*: `termloom`

* `--font-family <FONTS>`  
  Font family CSS fallback list used when generating SVG text elements.  
  *Default*: `"JetBrains Mono", "Fira Code", "Cascadia Code", "SF Mono", Menlo, Consolas, monospace`

* `--font-size <PX>`  
  Font size in pixels for SVG rendering.  
  *Default*: `14`

* `--line-height <FLOAT>`  
  Line height multiplier for character vertical alignment.  
  *Default*: `1.35`

* `--mask-secrets`  
  Enables the automatic redaction filter to mask common credentials (GitHub PATs, AWS access keys, Bearer tokens, private keys, and generic API keys) with asterisks in the rendered output.

* `--redact-regex <REGEX>`  
  Supplies custom regular expressions for masking sensitive data. Can be specified multiple times.

* `--trim-exit`  
  Automatically strips the final `exit` command input and prompt redraw before closing the recording session.

* `--hover-pause`  
  Adds a pure CSS `@media (hover: hover)` rule to the output SVG that pauses playback when the mouse cursor hovers over the window.

* `--font-url <URL>`  
  Injects an `@import url(...)` declaration into the SVG `<style>` tag, enabling external web fonts like Google Fonts.

* `--shadow`  
  Applies a smooth drop shadow to the terminal window frame in SVG output.

---

### 2. `termloom run`

Executes a scripted `.tape` automation file inside a pseudo-terminal. This produces deterministic, reproducible animations without manual keystroke timing.

```bash
termloom run [OPTIONS] <SCRIPT>
```

#### Arguments

* `<SCRIPT>`: Path to the `.tape` script file.

#### Options

* `-o, --output <OUTPUT>`  
  Destination file path. Overrides any `Output` directive defined within the tape script.

* `--theme <THEME>`  
  Overrides the palette specified in the script.

* `--window-style <STYLE>`  
  Overrides window frame style (`macos`, `squircle`, `none`).

* `--max-wait <SECS>`  
  Overrides idle frame clamp threshold.

* `--format <FORMAT>`  
  Forces output format (`svg`, `html`, `cast`).

* `--mask-secrets`  
  Enables credential masking during script execution.

* `--redact-regex <REGEX>`  
  Appends custom redaction regular expressions.

* `--trim-exit`  
  Trims any trailing exit sequence.

* `--hover-pause`  
  Enables hover-to-pause in generated SVG.

* `--font-url <URL>`  
  Imports custom web font stylesheet.

* `--shadow`  
  Enables window frame drop shadow.

---

### 3. `termloom snapshot`

Captures a high-resolution, single-frame vector SVG snapshot of a terminal state or command output.

```bash
termloom snapshot [OPTIONS] [OUTPUT]
```

#### Arguments

* `[OUTPUT]`: Destination path for the SVG snapshot. Defaults to `snapshot.svg`.

#### Options

* `-c, --command <CMD>`  
  Runs a command and renders its final output screen as an SVG snapshot.

* `--cols <COLS>`  
  Terminal column count. *Default*: `100`

* `--rows <ROWS>`  
  Terminal row count. *Default*: `28`

* `--theme <THEME>`  
  Color palette name. *Default*: `catppuccin-mocha`

* `--window-style <STYLE>`  
  Window frame style (`macos`, `squircle`, `none`). *Default*: `macos`

* `--title <TITLE>`  
  Header window title. *Default*: `termloom`

* `--font-family <FONTS>`  
  CSS font family fallback list.

* `--font-size <PX>`  
  Font size in pixels. *Default*: `14`

* `--font-url <URL>`  
  External font stylesheet import URL.

* `--shadow`  
  Enables window frame drop shadow.

* `--mask-secrets`  
  Masks secrets and credentials in the snapshot.

* `--redact-regex <REGEX>`  
  Custom regex patterns to redact.

---

### 4. `termloom convert`

Converts an existing Asciinema v2 (`.cast`) recording file into an animated vector SVG or interactive HTML player.

```bash
termloom convert [OPTIONS] <INPUT> -o <OUTPUT>
```

#### Arguments

* `<INPUT>`: Path to the source `.cast` file.

#### Options

* `-o, --output <OUTPUT>`  
  Required path for the converted output file.

* `--theme <THEME>`  
  Color palette to apply.  
  *Default*: `catppuccin-mocha`

* `--window-style <STYLE>`  
  Window frame style (`macos`, `squircle`, `none`).  
  *Default*: `macos`

* `--max-wait <SECS>`  
  Idle duration clamp applied during conversion.  
  *Default*: `1.5`

* `--format <FORMAT>`  
  Explicit output format (`svg`, `html`). Inferred by default from output filename.

* `--speed <MULTIPLIER>`  
  Playback speed multiplier applied across the entire timeline (e.g. `1.5` for 1.5x speed).  
  *Default*: `1.0`

---

## Tape Script Format

TermLoom supports `.tape` script files for deterministic automated recording:

```tape
# Output destination
Output demo.svg

# Window and layout settings
Set Width 90
Set Height 24
Set Theme catppuccin-mocha
Set FontSize 15
Set TypingSpeed 40ms

# Keystrokes and commands
Type "cargo build --release"
Enter
Sleep 2s
Type "termloom --version"
Enter
Sleep 1s
```

Supported tape commands:
* `Output <path>`: Output destination file.
* `Set <key> <val>`: Configure `Width`, `Height`, `Theme`, `FontSize`, `TypingSpeed`, `Title`, `WindowStyle`, `MaxWait`.
* `Type "<text>"`: Type text at configured typing speed.
* `Enter`: Send return key (`\r`).
* `Space [n]`: Send `n` spaces (default 1).
* `Backspace [n]`: Send `n` backspaces (default 1).
* `Sleep <duration>`: Pause execution (e.g., `500ms`, `2s`).
* `Ctrl+<c>`: Send control code (`Ctrl+C`, `Ctrl+D`, `Ctrl+L`).

---

## Exit Codes

* `0`: Operation completed successfully.
* `1`: General error (e.g. invalid arguments, file I/O failure).
* `2`: PTY allocation error or unsupported terminal environment.
