# TermLoom Color Themes

TermLoom provides built-in color themes covering popular terminal color schemes.

## Available Themes

### 1. `catppuccin-mocha` (Default Dark)
Dark palette based on Catppuccin Mocha.
* Background: `#1e1e2e`
* Foreground: `#cdd6f4`
* Cursor: `#f5e0dc`
* Accent: `#89b4fa`

### 2. `catppuccin-latte` (Light)
Light palette based on Catppuccin Latte.
* Background: `#eff1f5`
* Foreground: `#4c4f69`
* Cursor: `#dc8a78`
* Accent: `#1e66f5`

### 3. `dracula`
Dark theme with saturated accents.
* Background: `#282a36`
* Foreground: `#f8f8f2`
* Cursor: `#ff79c6`
* Accent: `#bd93f9`

### 4. `nord`
Arctic, north-bluish palette.
* Background: `#2e3440`
* Foreground: `#d8dee9`
* Cursor: `#88c0d0`
* Accent: `#81a1c1`

### 5. `tokyo-night`
Dark blue Tokyo night theme.
* Background: `#1a1b26`
* Foreground: `#c0caf5`
* Cursor: `#c0caf5`
* Accent: `#7aa2f7`

### 6. `gruvbox-dark`
Retro groove dark palette.
* Background: `#282828`
* Foreground: `#ebdbb2`
* Cursor: `#ebdbb2`
* Accent: `#d79921`

### 7. `solarized-dark`
Precision color palette designed by Ethan Schoonover.
* Background: `#002b36`
* Foreground: `#839496`
* Cursor: `#93a1a1`
* Accent: `#268bd2`

### 8. `monokai`
High-contrast vibrant theme.
* Background: `#272822`
* Foreground: `#f8f8f2`
* Cursor: `#f8f8f0`
* Accent: `#a6e22e`

### 9. `one-dark`
Atom's iconic One Dark syntax theme.
* Background: `#282c34`
* Foreground: `#abb2bf`
* Cursor: `#528bff`
* Accent: `#61afef`

## ANSI 16 Palette Mapping

Every theme defines:
* 8 normal ANSI colors: Black, Red, Green, Yellow, Blue, Magenta, Cyan, White.
* 8 bright ANSI colors: Bright Black, Bright Red, Bright Green, Bright Yellow, Bright Blue, Bright Magenta, Bright Cyan, Bright White.
* Default foreground and background colors.
* Cursor color.

When generating SVGs, these colors are declared in the root `<style>` block as CSS custom properties (`--c0` through `--c15`, `--fg`, `--bg`, `--cursor`), making the rendered output clean and inspectable.
