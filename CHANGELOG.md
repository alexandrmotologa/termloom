# Changelog

All notable changes to TermLoom will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-09-20

### Added
- Cross-platform PTY engine supporting Windows ConPTY and POSIX `openpty` via `portable-pty`.
- Virtual Terminal Emulator state machine powered by `vte` parser.
- 24-bit TrueColor, 256-color palette, and SGR text attribute support (bold, italic, underline, inverse, strikethrough).
- OSC 8 clickable hyperlink parsing in SVG and HTML outputs.
- Unicode character width handling with double-width emoji support via `unicode-width`.
- xxHash64 visual frame deduplication and `--max-wait` idle interval clamping.
- Zero-JavaScript animated SVG exporter using pure CSS `@keyframes`.
- Dark and light theme auto-adaptation via `@media (prefers-color-scheme: dark)`.
- Self-contained interactive HTML player with timeline scrubbing and one-click "Copy Command" button.
- Asciinema v2 (`.cast`) JSONL recording and conversion support.
- Built-in color palettes: `catppuccin-mocha`, `catppuccin-latte`, `dracula`, `nord`, `tokyo-night`, `gruvbox-dark`, `solarized-dark`, `monokai`, and `one-dark`.
- Command execution mode (`-c, --command`) for non-interactive and CI recording.
