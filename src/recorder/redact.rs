//! Credential and secret redaction filter for terminal screen grids.

use crate::emulator::grid::ScreenGrid;
use regex::Regex;

pub struct Redactor {
    patterns: Vec<Regex>,
}

impl Default for Redactor {
    fn default() -> Self {
        Self::standard()
    }
}

impl Redactor {
    /// Creates a Redactor with standard built-in secret patterns.
    pub fn standard() -> Self {
        let raw_patterns = [
            // GitHub Personal Access Tokens and OAuth tokens
            r"gh[pousr]_[A-Za-z0-9_]{36,}",
            // AWS Access Key IDs
            r"AKIA[0-9A-Z]{16}",
            // Slack API tokens
            r"xox[baprs]-[0-9a-zA-Z]{10,48}",
            // Bearer authorization tokens
            r"(?i)bearer\s+[a-zA-Z0-9_\-\.]{20,}",
            // Private keys
            r"-----BEGIN [A-Z ]+ PRIVATE KEY-----",
            // Generic API key assignments: api_key=..., secret=...
            r#"(?i)(api[_-]?key|secret|password|access[_-]?token)\s*[:=]\s*['"]?[a-zA-Z0-9_\-\.]{16,}['"]?"#,
        ];

        let patterns = raw_patterns
            .iter()
            .filter_map(|p| Regex::new(p).ok())
            .collect();

        Self { patterns }
    }

    /// Creates a Redactor with additional custom user-supplied regexes.
    pub fn with_custom(custom_regexes: &[String]) -> Result<Self, regex::Error> {
        let mut redactor = Self::standard();
        for r in custom_regexes {
            let re = Regex::new(r)?;
            redactor.patterns.push(re);
        }
        Ok(redactor)
    }

    /// Redacts matched secrets in place across all cells in the screen grid.
    /// Returns the number of secret spans redacted.
    pub fn redact_grid(&self, grid: &mut ScreenGrid) -> usize {
        let mut count = 0;

        for r in 0..grid.rows {
            // Build text line and character index mapping
            let mut line_text = String::new();
            let mut col_map = Vec::new(); // maps byte position in line_text to column index

            for (c, cell) in grid.cells[r].iter().enumerate() {
                if cell.is_continuation {
                    continue;
                }
                let start_byte = line_text.len();
                line_text.push_str(&cell.grapheme);
                let end_byte = line_text.len();
                for _ in start_byte..end_byte {
                    col_map.push(c);
                }
            }

            if line_text.trim().is_empty() {
                continue;
            }

            for re in &self.patterns {
                for mat in re.find_iter(&line_text) {
                    count += 1;
                    let start_col = if mat.start() < col_map.len() {
                        col_map[mat.start()]
                    } else {
                        0
                    };
                    let end_col = if mat.end() <= col_map.len() {
                        col_map[mat.end() - 1]
                    } else {
                        grid.cols.saturating_sub(1)
                    };

                    for c in start_col..=end_col.min(grid.cols - 1) {
                        if !grid.cells[r][c].is_continuation && !grid.cells[r][c].is_empty() {
                            grid.cells[r][c].grapheme = "*".to_string();
                        }
                    }
                }
            }
        }

        count
    }
}
