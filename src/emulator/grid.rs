//! 2D virtual terminal screen grid representation.

use super::cell::{Cell, StyleState};
use unicode_width::UnicodeWidthChar;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScreenGrid {
    pub cols: usize,
    pub rows: usize,
    pub cells: Vec<Vec<Cell>>,
    pub cursor_col: usize,
    pub cursor_row: usize,
    pub cursor_visible: bool,
    pub saved_cursor: (usize, usize),
}

impl ScreenGrid {
    pub fn new(cols: usize, rows: usize) -> Self {
        let cols = cols.max(1);
        let rows = rows.max(1);
        let cells = vec![vec![Cell::blank(); cols]; rows];
        Self {
            cols,
            rows,
            cells,
            cursor_col: 0,
            cursor_row: 0,
            cursor_visible: true,
            saved_cursor: (0, 0),
        }
    }

    pub fn resize(&mut self, new_cols: usize, new_rows: usize) {
        let new_cols = new_cols.max(1);
        let new_rows = new_rows.max(1);
        let mut new_cells = vec![vec![Cell::blank(); new_cols]; new_rows];

        let copy_rows = self.rows.min(new_rows);
        let copy_cols = self.cols.min(new_cols);

        for (r, row) in new_cells.iter_mut().enumerate().take(copy_rows) {
            for (c, cell) in row.iter_mut().enumerate().take(copy_cols) {
                *cell = self.cells[r][c].clone();
            }
        }

        self.cols = new_cols;
        self.rows = new_rows;
        self.cells = new_cells;
        self.cursor_col = self.cursor_col.min(new_cols - 1);
        self.cursor_row = self.cursor_row.min(new_rows - 1);
    }

    pub fn clear_all(&mut self) {
        for row in &mut self.cells {
            for cell in row {
                *cell = Cell::blank();
            }
        }
    }

    pub fn clear_line(&mut self, row: usize, mode: u8) {
        if row >= self.rows {
            return;
        }
        let col = self.cursor_col.min(self.cols);
        match mode {
            0 => {
                // Clear from cursor to end of line
                for c in col..self.cols {
                    self.cells[row][c] = Cell::blank();
                }
            }
            1 => {
                // Clear from start of line to cursor
                for c in 0..=col.min(self.cols - 1) {
                    self.cells[row][c] = Cell::blank();
                }
            }
            2 => {
                // Clear entire line
                for c in 0..self.cols {
                    self.cells[row][c] = Cell::blank();
                }
            }
            _ => {}
        }
    }

    pub fn clear_display(&mut self, mode: u8) {
        match mode {
            0 => {
                // Clear from cursor to end of screen
                self.clear_line(self.cursor_row, 0);
                for r in (self.cursor_row + 1)..self.rows {
                    for c in 0..self.cols {
                        self.cells[r][c] = Cell::blank();
                    }
                }
            }
            1 => {
                // Clear from beginning of screen to cursor
                for r in 0..self.cursor_row {
                    for c in 0..self.cols {
                        self.cells[r][c] = Cell::blank();
                    }
                }
                self.clear_line(self.cursor_row, 1);
            }
            2 | 3 => {
                // Clear entire screen
                self.clear_all();
            }
            _ => {}
        }
    }

    pub fn scroll_up(&mut self) {
        if self.rows <= 1 {
            return;
        }
        self.cells.remove(0);
        self.cells.push(vec![Cell::blank(); self.cols]);
    }

    pub fn scroll_down(&mut self) {
        if self.rows <= 1 {
            return;
        }
        self.cells.pop();
        self.cells.insert(0, vec![Cell::blank(); self.cols]);
    }

    pub fn carriage_return(&mut self) {
        self.cursor_col = 0;
    }

    pub fn line_feed(&mut self) {
        if self.cursor_row + 1 >= self.rows {
            self.scroll_up();
        } else {
            self.cursor_row += 1;
        }
    }

    pub fn backspace(&mut self) {
        if self.cursor_col > 0 {
            self.cursor_col -= 1;
        }
    }

    pub fn tab(&mut self) {
        // Tab stops every 8 columns
        let next_tab = (self.cursor_col / 8 + 1) * 8;
        self.cursor_col = next_tab.min(self.cols - 1);
    }

    pub fn set_cursor(&mut self, col: usize, row: usize) {
        self.cursor_col = col.min(self.cols.saturating_sub(1));
        self.cursor_row = row.min(self.rows.saturating_sub(1));
    }

    pub fn move_cursor(&mut self, delta_col: isize, delta_row: isize) {
        let new_col = (self.cursor_col as isize + delta_col)
            .max(0)
            .min((self.cols.saturating_sub(1)) as isize) as usize;
        let new_row = (self.cursor_row as isize + delta_row)
            .max(0)
            .min((self.rows.saturating_sub(1)) as isize) as usize;
        self.cursor_col = new_col;
        self.cursor_row = new_row;
    }

    pub fn save_cursor(&mut self) {
        self.saved_cursor = (self.cursor_col, self.cursor_row);
    }

    pub fn restore_cursor(&mut self) {
        self.set_cursor(self.saved_cursor.0, self.saved_cursor.1);
    }

    pub fn put_char(&mut self, ch: char, style: &StyleState) {
        // Auto-wrap if cursor exceeded columns
        if self.cursor_col >= self.cols {
            self.cursor_col = 0;
            self.line_feed();
        }

        let char_width = UnicodeWidthChar::width(ch).unwrap_or(1);

        if char_width == 0 {
            // Combining character: append to previous cell grapheme if available
            if self.cursor_col > 0 {
                let prev_col = self.cursor_col - 1;
                let cell = &mut self.cells[self.cursor_row][prev_col];
                if !cell.is_continuation {
                    cell.grapheme.push(ch);
                    return;
                } else if prev_col > 0 {
                    let base_col = prev_col - 1;
                    self.cells[self.cursor_row][base_col].grapheme.push(ch);
                    return;
                }
            }
            return;
        }

        let cell = Cell {
            grapheme: ch.to_string(),
            width: char_width as u8,
            fg: style.fg,
            bg: style.bg,
            bold: style.bold,
            italic: style.italic,
            underline: style.underline,
            inverse: style.inverse,
            strikethrough: style.strikethrough,
            link: style.link.clone(),
            is_continuation: false,
        };

        self.cells[self.cursor_row][self.cursor_col] = cell;

        if char_width == 2 {
            if self.cursor_col + 1 < self.cols {
                self.cells[self.cursor_row][self.cursor_col + 1] = Cell::continuation();
                self.cursor_col += 2;
            } else {
                self.cursor_col += 1;
            }
        } else {
            self.cursor_col += 1;
        }
    }
}
