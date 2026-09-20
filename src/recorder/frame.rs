//! Snapshot frame representation and 64-bit visual hashing.

use crate::emulator::grid::ScreenGrid;
use xxhash_rust::xxh64::xxh64;

#[derive(Debug, Clone)]
pub struct Frame {
    /// Screen grid state at this frame.
    pub grid: ScreenGrid,
    /// Duration this frame is displayed in seconds.
    pub duration: f64,
    /// Timestamp offset in seconds from animation start.
    pub timestamp: f64,
    /// 64-bit visual state hash.
    pub hash: u64,
}

impl Frame {
    pub fn new(grid: ScreenGrid, duration: f64, timestamp: f64) -> Self {
        let hash = compute_grid_hash(&grid);
        Self {
            grid,
            duration,
            timestamp,
            hash,
        }
    }
}

pub fn compute_grid_hash(grid: &ScreenGrid) -> u64 {
    let mut bytes = Vec::with_capacity(grid.cols * grid.rows * 8);

    // Encode cursor position and visibility
    bytes.push(if grid.cursor_visible { 1 } else { 0 });
    bytes.extend_from_slice(&(grid.cursor_col as u32).to_le_bytes());
    bytes.extend_from_slice(&(grid.cursor_row as u32).to_le_bytes());

    // Encode all cells
    for row in &grid.cells {
        for cell in row {
            bytes.extend_from_slice(cell.grapheme.as_bytes());
            bytes.push(cell.width);
            encode_color(cell.fg, &mut bytes);
            encode_color(cell.bg, &mut bytes);

            let mut flags = 0u8;
            if cell.bold {
                flags |= 1;
            }
            if cell.italic {
                flags |= 2;
            }
            if cell.underline {
                flags |= 4;
            }
            if cell.inverse {
                flags |= 8;
            }
            if cell.strikethrough {
                flags |= 16;
            }
            if cell.is_continuation {
                flags |= 32;
            }
            bytes.push(flags);

            if let Some(ref link) = cell.link {
                bytes.extend_from_slice(link.as_bytes());
            }
        }
    }

    xxh64(&bytes, 0)
}

fn encode_color(color: crate::emulator::color::Color, bytes: &mut Vec<u8>) {
    match color {
        crate::emulator::color::Color::DefaultFg => bytes.push(0),
        crate::emulator::color::Color::DefaultBg => bytes.push(1),
        crate::emulator::color::Color::Indexed(idx) => {
            bytes.push(2);
            bytes.push(idx);
        }
        crate::emulator::color::Color::Rgb(r, g, b) => {
            bytes.push(3);
            bytes.push(r);
            bytes.push(g);
            bytes.push(b);
        }
    }
}
