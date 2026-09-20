//! Terminal grid cell definition and text styles.

use super::color::Color;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cell {
    pub grapheme: String,
    pub width: u8,
    pub fg: Color,
    pub bg: Color,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub inverse: bool,
    pub strikethrough: bool,
    pub link: Option<String>,
    pub is_continuation: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            grapheme: " ".to_string(),
            width: 1,
            fg: Color::DefaultFg,
            bg: Color::DefaultBg,
            bold: false,
            italic: false,
            underline: false,
            inverse: false,
            strikethrough: false,
            link: None,
            is_continuation: false,
        }
    }
}

impl Cell {
    pub fn blank() -> Self {
        Self::default()
    }

    pub fn continuation() -> Self {
        Self {
            grapheme: String::new(),
            width: 0,
            is_continuation: true,
            ..Default::default()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.grapheme.is_empty() || self.grapheme == " "
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StyleState {
    pub fg: Color,
    pub bg: Color,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub inverse: bool,
    pub strikethrough: bool,
    pub link: Option<String>,
}

impl Default for StyleState {
    fn default() -> Self {
        Self {
            fg: Color::DefaultFg,
            bg: Color::DefaultBg,
            bold: false,
            italic: false,
            underline: false,
            inverse: false,
            strikethrough: false,
            link: None,
        }
    }
}

impl StyleState {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
