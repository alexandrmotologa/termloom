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

---

### 2. `termloom convert`

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

## Exit Codes

* `0`: Operation completed successfully.
* `1`: General error (e.g. invalid arguments, file I/O failure).
* `2`: PTY allocation error or unsupported terminal environment.
