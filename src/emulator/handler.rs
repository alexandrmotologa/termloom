//! VTE parser Perform callback implementation for TermLoom.

use super::cell::StyleState;
use super::color::Color;
use super::grid::ScreenGrid;
use vte::{Params, Perform};

pub struct VteHandler {
    pub grid: ScreenGrid,
    pub style: StyleState,
}

impl VteHandler {
    pub fn new(cols: usize, rows: usize) -> Self {
        Self {
            grid: ScreenGrid::new(cols, rows),
            style: StyleState::default(),
        }
    }

    fn handle_sgr(&mut self, params: &Params) {
        if params.is_empty() {
            self.style.reset();
            return;
        }

        let mut iter = params.iter().map(|sub| sub[0]);
        while let Some(code) = iter.next() {
            match code {
                0 => self.style.reset(),
                1 => self.style.bold = true,
                3 => self.style.italic = true,
                4 => self.style.underline = true,
                7 => self.style.inverse = true,
                9 => self.style.strikethrough = true,
                22 => self.style.bold = false,
                23 => self.style.italic = false,
                24 => self.style.underline = false,
                27 => self.style.inverse = false,
                29 => self.style.strikethrough = false,
                // Standard ANSI Foreground
                30..=37 => self.style.fg = Color::Indexed((code - 30) as u8),
                // Extended Foreground
                38 => {
                    if let Some(mode) = iter.next() {
                        match mode {
                            5 => {
                                if let Some(idx) = iter.next() {
                                    self.style.fg = Color::Indexed(idx as u8);
                                }
                            }
                            2 => {
                                let r = iter.next();
                                let g = iter.next();
                                let b = iter.next();
                                if let (Some(r), Some(g), Some(b)) = (r, g, b) {
                                    self.style.fg = Color::Rgb(r as u8, g as u8, b as u8);
                                }
                            }
                            _ => {}
                        }
                    }
                }
                39 => self.style.fg = Color::DefaultFg,
                // Standard ANSI Background
                40..=47 => self.style.bg = Color::Indexed((code - 40) as u8),
                // Extended Background
                48 => {
                    if let Some(mode) = iter.next() {
                        match mode {
                            5 => {
                                if let Some(idx) = iter.next() {
                                    self.style.bg = Color::Indexed(idx as u8);
                                }
                            }
                            2 => {
                                let r = iter.next();
                                let g = iter.next();
                                let b = iter.next();
                                if let (Some(r), Some(g), Some(b)) = (r, g, b) {
                                    self.style.bg = Color::Rgb(r as u8, g as u8, b as u8);
                                }
                            }
                            _ => {}
                        }
                    }
                }
                49 => self.style.bg = Color::DefaultBg,
                // Bright Foreground
                90..=97 => self.style.fg = Color::Indexed((8 + code - 90) as u8),
                // Bright Background
                100..=107 => self.style.bg = Color::Indexed((8 + code - 100) as u8),
                _ => {}
            }
        }
    }
}

impl Perform for VteHandler {
    fn print(&mut self, c: char) {
        self.grid.put_char(c, &self.style);
    }

    fn execute(&mut self, byte: u8) {
        match byte {
            b'\n' => self.grid.line_feed(),
            b'\r' => self.grid.carriage_return(),
            b'\x08' => self.grid.backspace(),
            b'\t' => self.grid.tab(),
            _ => {}
        }
    }

    fn hook(&mut self, _params: &Params, _intermediates: &[u8], _ignore: bool, _action: char) {}
    fn put(&mut self, _byte: u8) {}
    fn unhook(&mut self) {}

    fn osc_dispatch(&mut self, params: &[&[u8]], _bell_terminated: bool) {
        if params.is_empty() {
            return;
        }

        // OSC 8: Hyperlinks -> \x1b]8;params;url\x1b\
        if params[0] == b"8" {
            if params.len() >= 3 && !params[2].is_empty() {
                if let Ok(url) = std::str::from_utf8(params[2]) {
                    self.style.link = Some(url.to_string());
                }
            } else {
                self.style.link = None;
            }
        }
    }

    fn csi_dispatch(&mut self, params: &Params, intermediates: &[u8], _ignore: bool, action: char) {
        let first_param = |default: usize| -> usize {
            params
                .iter()
                .next()
                .map(|p| p[0] as usize)
                .filter(|&v| v > 0)
                .unwrap_or(default)
        };

        match action {
            // SGR - Select Graphic Rendition
            'm' => self.handle_sgr(params),

            // CUU - Cursor Up
            'A' => {
                let count = first_param(1);
                self.grid.move_cursor(0, -(count as isize));
            }

            // CUD - Cursor Down
            'B' => {
                let count = first_param(1);
                self.grid.move_cursor(0, count as isize);
            }

            // CUF - Cursor Forward
            'C' => {
                let count = first_param(1);
                self.grid.move_cursor(count as isize, 0);
            }

            // CUB - Cursor Back
            'D' => {
                let count = first_param(1);
                self.grid.move_cursor(-(count as isize), 0);
            }

            // CNL - Cursor Next Line
            'E' => {
                let count = first_param(1);
                self.grid.cursor_col = 0;
                self.grid.move_cursor(0, count as isize);
            }

            // CPL - Cursor Previous Line
            'F' => {
                let count = first_param(1);
                self.grid.cursor_col = 0;
                self.grid.move_cursor(0, -(count as isize));
            }

            // CHA - Cursor Horizontal Absolute
            'G' => {
                let col = first_param(1).saturating_sub(1);
                self.grid.cursor_col = col.min(self.grid.cols.saturating_sub(1));
            }

            // CUP - Cursor Position
            'H' | 'f' => {
                let row = params
                    .iter()
                    .next()
                    .map(|p| p[0] as usize)
                    .filter(|&v| v > 0)
                    .unwrap_or(1)
                    .saturating_sub(1);
                let col = params
                    .iter()
                    .nth(1)
                    .map(|p| p[0] as usize)
                    .filter(|&v| v > 0)
                    .unwrap_or(1)
                    .saturating_sub(1);
                self.grid.set_cursor(col, row);
            }

            // ED - Erase in Display
            'J' => {
                let mode = params.iter().next().map(|p| p[0] as u8).unwrap_or(0);
                self.grid.clear_display(mode);
            }

            // EL - Erase in Line
            'K' => {
                let mode = params.iter().next().map(|p| p[0] as u8).unwrap_or(0);
                self.grid.clear_line(self.grid.cursor_row, mode);
            }

            // Cursor Save / Restore
            's' => self.grid.save_cursor(),
            'u' => self.grid.restore_cursor(),

            // Private modes (?...)
            'h' | 'l' if intermediates == b"?" => {
                let enable = action == 'h';
                for param in params.iter() {
                    if param[0] == 25 {
                        // DECTCEM: Show/Hide Cursor
                        self.grid.cursor_visible = enable;
                    }
                }
            }
            _ => {}
        }
    }

    fn esc_dispatch(&mut self, _intermediates: &[u8], _ignore: bool, byte: u8) {
        match byte {
            b'7' => self.grid.save_cursor(),
            b'8' => self.grid.restore_cursor(),
            b'c' => {
                self.grid.clear_all();
                self.grid.set_cursor(0, 0);
                self.style.reset();
            }
            _ => {}
        }
    }
}
