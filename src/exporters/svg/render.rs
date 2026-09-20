//! Pure vector animated SVG exporter using embedded CSS keyframes.

use super::window::{xml_escape, Geometry};
use crate::cli::WindowStyle;
use crate::emulator::cell::Cell;
use crate::emulator::color::Color;
use crate::recorder::frame::Frame;
use crate::themes::Palette;

pub struct SvgRenderer<'a> {
    palette: &'a Palette,
    window_style: WindowStyle,
    title: &'a str,
    font_family: &'a str,
    font_size: u32,
    line_height: f64,
}

impl<'a> SvgRenderer<'a> {
    pub fn new(
        palette: &'a Palette,
        window_style: WindowStyle,
        title: &'a str,
        font_family: &'a str,
        font_size: u32,
        line_height: f64,
    ) -> Self {
        Self {
            palette,
            window_style,
            title,
            font_family,
            font_size,
            line_height,
        }
    }

    pub fn render(&self, frames: &[Frame], cols: usize, rows: usize) -> String {
        let geom = Geometry::calculate(
            cols,
            rows,
            self.font_size,
            self.line_height,
            self.window_style,
        );
        let total_duration: f64 = frames.iter().map(|f| f.duration).sum::<f64>().max(0.1);

        let mut svg = String::with_capacity(frames.len() * cols * rows * 12 + 4096);

        // Header and SVG root
        svg.push_str(&format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 {width} {height}" width="100%" height="100%">
"#,
            width = geom.total_width,
            height = geom.total_height,
        ));

        // Style and CSS keyframes
        svg.push_str("  <style>\n");
        svg.push_str(&format!(
            r#"    :root {{
      --bg: {bg};
      --fg: {fg};
      --cursor: {cursor};
    }}
    text {{
      font-family: {font_family};
      font-size: {font_size}px;
      dominant-baseline: alphabetic;
      white-space: pre;
    }}
    .terminal-bg {{
      fill: var(--bg);
    }}
    .frame {{
      visibility: hidden;
    }}
"#,
            bg = Palette::hex(self.palette.bg),
            fg = Palette::hex(self.palette.fg),
            cursor = Palette::hex(self.palette.cursor),
            font_family = self.font_family,
            font_size = self.font_size,
        ));

        // Generate keyframe animations for each frame
        let mut cur_time = 0.0;
        for (i, frame) in frames.iter().enumerate() {
            let start_pct = (cur_time / total_duration) * 100.0;
            let end_pct = ((cur_time + frame.duration) / total_duration) * 100.0;
            cur_time += frame.duration;

            let anim_name = format!("f_{}", i);
            svg.push_str(&format!(
                "    .frame_{i} {{ animation: {anim_name} {total_duration:.3}s infinite; }}\n",
                i = i,
                anim_name = anim_name,
                total_duration = total_duration,
            ));

            svg.push_str(&format!("    @keyframes {} {{\n", anim_name));
            if i == 0 {
                svg.push_str(&format!(
                    "      0.0% {{ visibility: visible; }}\n      {:.2}% {{ visibility: visible; }}\n      {:.2}% {{ visibility: hidden; }}\n      100.0% {{ visibility: hidden; }}\n",
                    end_pct,
                    (end_pct + 0.01).min(100.0)
                ));
            } else if i + 1 == frames.len() {
                svg.push_str(&format!(
                    "      0.0% {{ visibility: hidden; }}\n      {:.2}% {{ visibility: hidden; }}\n      {:.2}% {{ visibility: visible; }}\n      100.0% {{ visibility: visible; }}\n",
                    (start_pct - 0.01).max(0.0),
                    start_pct
                ));
            } else {
                svg.push_str(&format!(
                    "      0.0% {{ visibility: hidden; }}\n      {:.2}% {{ visibility: hidden; }}\n      {:.2}% {{ visibility: visible; }}\n      {:.2}% {{ visibility: visible; }}\n      {:.2}% {{ visibility: hidden; }}\n      100.0% {{ visibility: hidden; }}\n",
                    (start_pct - 0.01).max(0.0),
                    start_pct,
                    end_pct,
                    (end_pct + 0.01).min(100.0)
                ));
            }
            svg.push_str("    }\n");
        }

        svg.push_str("  </style>\n\n");

        // Window Chrome
        svg.push_str(&geom.render_chrome(
            self.window_style,
            self.title,
            &Palette::hex(self.palette.bg),
        ));

        // Render each frame
        let x_offset = geom.padding_x;
        let y_offset = geom.header_height + geom.padding_y;

        for (i, frame) in frames.iter().enumerate() {
            svg.push_str(&format!("  <!-- Frame {} -->\n", i));
            svg.push_str(&format!("  <g class=\"frame frame_{}\">\n", i));

            // Cell backgrounds if any
            for (r, row) in frame.grid.cells.iter().enumerate() {
                for (c, cell) in row.iter().enumerate() {
                    if cell.is_continuation {
                        continue;
                    }
                    if cell.bg != Color::DefaultBg {
                        let cx = x_offset + c as f64 * geom.char_width;
                        let cy = y_offset + r as f64 * geom.char_height;
                        let w = geom.char_width * (cell.width.max(1) as f64);
                        let fill = cell.bg.to_hex(self.palette, false);
                        svg.push_str(&format!(
                            r#"    <rect x="{cx:.1}" y="{cy:.1}" width="{w:.1}" height="{h:.1}" fill="{fill}" />
"#,
                            cx = cx,
                            cy = cy,
                            w = w,
                            h = geom.char_height,
                            fill = fill,
                        ));
                    }
                }
            }

            // Lines and text runs
            for (r, row) in frame.grid.cells.iter().enumerate() {
                let y = y_offset + (r as f64 + 0.82) * geom.char_height;
                let mut runs = cluster_row(row);

                if !runs.is_empty() {
                    svg.push_str(&format!(
                        r#"    <text x="{x:.1}" y="{y:.1}">"#,
                        x = x_offset,
                        y = y
                    ));
                    for run in runs.drain(..) {
                        let text_content = xml_escape(&run.text);
                        let fg_hex = run.fg.to_hex(self.palette, true);

                        let mut style_attrs = String::new();
                        if run.bold {
                            style_attrs.push_str(" font-weight=\"bold\"");
                        }
                        if run.italic {
                            style_attrs.push_str(" font-style=\"italic\"");
                        }
                        if run.underline && run.strikethrough {
                            style_attrs.push_str(" text-decoration=\"underline line-through\"");
                        } else if run.underline {
                            style_attrs.push_str(" text-decoration=\"underline\"");
                        } else if run.strikethrough {
                            style_attrs.push_str(" text-decoration=\"line-through\"");
                        }

                        let tspan = format!(
                            r#"<tspan fill="{fg}"{attrs}>{content}</tspan>"#,
                            fg = fg_hex,
                            attrs = style_attrs,
                            content = text_content
                        );

                        if let Some(ref link) = run.link {
                            svg.push_str(&format!(
                                r#"<a xlink:href="{url}" target="_blank">{tspan}</a>"#,
                                url = xml_escape(link),
                                tspan = tspan
                            ));
                        } else {
                            svg.push_str(&tspan);
                        }
                    }
                    svg.push_str("</text>\n");
                }
            }

            // Cursor
            if frame.grid.cursor_visible
                && frame.grid.cursor_col < cols
                && frame.grid.cursor_row < rows
            {
                let cx = x_offset + frame.grid.cursor_col as f64 * geom.char_width;
                let cy = y_offset + frame.grid.cursor_row as f64 * geom.char_height;
                svg.push_str(&format!(
                    r#"    <rect x="{cx:.1}" y="{cy:.1}" width="{w:.1}" height="{h:.1}" fill="var(--cursor)" opacity="0.75" />
"#,
                    cx = cx,
                    cy = cy,
                    w = geom.char_width,
                    h = geom.char_height,
                ));
            }

            svg.push_str("  </g>\n");
        }

        svg.push_str("</svg>\n");
        svg
    }
}

struct TextRun {
    text: String,
    fg: Color,
    bold: bool,
    italic: bool,
    underline: bool,
    strikethrough: bool,
    link: Option<String>,
}

fn cluster_row(row: &[Cell]) -> Vec<TextRun> {
    let mut runs = Vec::new();
    let mut current_run: Option<TextRun> = None;

    for cell in row {
        if cell.is_continuation {
            continue;
        }

        let effective_fg = if cell.inverse { cell.bg } else { cell.fg };

        let matches = if let Some(ref r) = current_run {
            r.fg == effective_fg
                && r.bold == cell.bold
                && r.italic == cell.italic
                && r.underline == cell.underline
                && r.strikethrough == cell.strikethrough
                && r.link == cell.link
        } else {
            false
        };

        if matches {
            if let Some(ref mut r) = current_run {
                r.text.push_str(&cell.grapheme);
            }
        } else {
            if let Some(r) = current_run.take() {
                runs.push(r);
            }
            current_run = Some(TextRun {
                text: cell.grapheme.clone(),
                fg: effective_fg,
                bold: cell.bold,
                italic: cell.italic,
                underline: cell.underline,
                strikethrough: cell.strikethrough,
                link: cell.link.clone(),
            });
        }
    }

    if let Some(r) = current_run {
        runs.push(r);
    }

    // Trim trailing empty spaces to reduce SVG size
    while let Some(last) = runs.last() {
        if last.text.trim().is_empty() && last.link.is_none() {
            runs.pop();
        } else {
            break;
        }
    }

    runs
}
