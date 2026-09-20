//! Window chrome and geometry calculation for SVG exports.

use crate::cli::WindowStyle;

#[derive(Debug, Clone)]
pub struct Geometry {
    pub char_width: f64,
    pub char_height: f64,
    pub header_height: f64,
    pub padding_x: f64,
    pub padding_y: f64,
    pub total_width: f64,
    pub total_height: f64,
    pub border_radius: f64,
}

impl Geometry {
    pub fn calculate(
        cols: usize,
        rows: usize,
        font_size: u32,
        line_height: f64,
        style: WindowStyle,
    ) -> Self {
        // Monospace standard ratio (approx 0.605 of font size)
        let char_width = (font_size as f64) * 0.605;
        let char_height = (font_size as f64) * line_height;

        let (header_height, border_radius) = match style {
            WindowStyle::Macos => (38.0, 12.0),
            WindowStyle::Squircle => (32.0, 8.0),
            WindowStyle::None => (0.0, 0.0),
        };

        let padding_x = if style == WindowStyle::None {
            8.0
        } else {
            16.0
        };
        let padding_y = if style == WindowStyle::None {
            8.0
        } else {
            16.0
        };

        let total_width = (cols as f64 * char_width + padding_x * 2.0).ceil();
        let total_height = (header_height + rows as f64 * char_height + padding_y * 2.0).ceil();

        Self {
            char_width,
            char_height,
            header_height,
            padding_x,
            padding_y,
            total_width,
            total_height,
            border_radius,
        }
    }

    pub fn render_chrome(&self, style: WindowStyle, title: &str, bg_hex: &str) -> String {
        match style {
            WindowStyle::Macos => {
                format!(
                    r##"  <!-- Window Frame -->
  <rect width="{width}" height="{height}" rx="{rx}" fill="{bg}" stroke="#334155" stroke-width="1.5" />
  <!-- Traffic Light Controls -->
  <circle cx="20" cy="19" r="6" fill="#ef4444" />
  <circle cx="38" cy="19" r="6" fill="#f59e0b" />
  <circle cx="56" cy="19" r="6" fill="#10b981" />
  <!-- Title -->
  <text x="{half_width}" y="23" fill="#94a3b8" font-family="-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif" font-size="12" font-weight="600" text-anchor="middle">{title}</text>
  <line x1="0" y1="{header_h}" x2="{width}" y2="{header_h}" stroke="#334155" stroke-width="1" />
"##,
                    width = self.total_width,
                    height = self.total_height,
                    rx = self.border_radius,
                    bg = bg_hex,
                    half_width = self.total_width / 2.0,
                    title = xml_escape(title),
                    header_h = self.header_height,
                )
            }
            WindowStyle::Squircle => {
                format!(
                    r##"  <!-- Window Frame -->
  <rect width="{width}" height="{height}" rx="{rx}" fill="{bg}" stroke="#334155" stroke-width="1" />
  <!-- Title -->
  <text x="{half_width}" y="20" fill="#94a3b8" font-family="-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif" font-size="11" font-weight="500" text-anchor="middle">{title}</text>
  <line x1="0" y1="{header_h}" x2="{width}" y2="{header_h}" stroke="#334155" stroke-width="1" />
"##,
                    width = self.total_width,
                    height = self.total_height,
                    rx = self.border_radius,
                    bg = bg_hex,
                    half_width = self.total_width / 2.0,
                    title = xml_escape(title),
                    header_h = self.header_height,
                )
            }
            WindowStyle::None => {
                format!(
                    r##"  <rect width="{width}" height="{height}" fill="{bg}" />
"##,
                    width = self.total_width,
                    height = self.total_height,
                    bg = bg_hex,
                )
            }
        }
    }
}

pub fn xml_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(c),
        }
    }
    out
}
