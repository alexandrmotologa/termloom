//! Color representations for terminal cells.

use crate::themes::Palette;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    DefaultFg,
    DefaultBg,
    Indexed(u8),
    Rgb(u8, u8, u8),
}

impl Color {
    /// Resolve this color to an RGB tuple using the provided palette.
    pub fn resolve(&self, palette: &Palette, _is_fg: bool) -> (u8, u8, u8) {
        match *self {
            Color::DefaultFg => palette.fg,
            Color::DefaultBg => palette.bg,
            Color::Indexed(idx) => palette.resolve_256(idx),
            Color::Rgb(r, g, b) => (r, g, b),
        }
    }

    /// Convert to CSS hex string.
    pub fn to_hex(&self, palette: &Palette, is_fg: bool) -> String {
        let (r, g, b) = self.resolve(palette, is_fg);
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    }
}
