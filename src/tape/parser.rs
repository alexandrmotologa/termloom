//! Tape file parser for scripted terminal sessions.

use crate::cli::WindowStyle;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub enum TapeCommand {
    Type(String),
    Enter,
    Space(usize),
    Backspace(usize),
    Sleep(Duration),
    Ctrl(char),
}

#[derive(Debug, Clone)]
pub struct TapeScript {
    pub output: Option<PathBuf>,
    pub cols: u16,
    pub rows: u16,
    pub theme: String,
    pub window_style: WindowStyle,
    pub max_wait: f64,
    pub title: String,
    pub font_size: u32,
    pub typing_speed: Duration,
    pub commands: Vec<TapeCommand>,
}

impl Default for TapeScript {
    fn default() -> Self {
        Self {
            output: None,
            cols: 100,
            rows: 28,
            theme: "catppuccin-mocha".to_string(),
            window_style: WindowStyle::Macos,
            max_wait: 1.5,
            title: "termloom".to_string(),
            font_size: 14,
            typing_speed: Duration::from_millis(50),
            commands: Vec::new(),
        }
    }
}

pub fn parse_tape(content: &str) -> Result<TapeScript, String> {
    let mut script = TapeScript::default();

    for (line_idx, line) in content.lines().enumerate() {
        let line_num = line_idx + 1;
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("//") {
            continue;
        }

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        let keyword = parts[0];

        match keyword.to_ascii_lowercase().as_str() {
            "output" => {
                if parts.len() < 2 {
                    return Err(format!("Line {}: Output requires a file path", line_num));
                }
                let raw_path = trimmed[keyword.len()..].trim().trim_matches('"');
                script.output = Some(PathBuf::from(raw_path));
            }
            "set" => {
                if parts.len() < 3 {
                    return Err(format!("Line {}: Set requires a key and value", line_num));
                }
                let key = parts[1].to_ascii_lowercase();
                let val = trimmed[keyword.len()..].trim()[parts[1].len()..]
                    .trim()
                    .trim_matches('"');

                match key.as_str() {
                    "theme" => script.theme = val.to_string(),
                    "width" | "cols" => {
                        script.cols = val
                            .parse()
                            .map_err(|_| format!("Line {}: Invalid width", line_num))?;
                    }
                    "height" | "rows" => {
                        script.rows = val
                            .parse()
                            .map_err(|_| format!("Line {}: Invalid height", line_num))?;
                    }
                    "maxwait" | "max_wait" => {
                        script.max_wait = val
                            .parse()
                            .map_err(|_| format!("Line {}: Invalid max_wait", line_num))?;
                    }
                    "title" => script.title = val.to_string(),
                    "fontsize" | "font_size" => {
                        script.font_size = val
                            .parse()
                            .map_err(|_| format!("Line {}: Invalid font_size", line_num))?;
                    }
                    "windowstyle" | "window_style" => {
                        script.window_style = match val.to_ascii_lowercase().as_str() {
                            "squircle" => WindowStyle::Squircle,
                            "none" => WindowStyle::None,
                            _ => WindowStyle::Macos,
                        };
                    }
                    "typingspeed" | "typing_speed" => {
                        script.typing_speed = parse_duration(val).ok_or_else(|| {
                            format!("Line {}: Invalid duration format for TypingSpeed", line_num)
                        })?;
                    }
                    _ => {}
                }
            }
            "type" => {
                let text = trimmed[keyword.len()..].trim();
                let stripped = if text.starts_with('"') && text.ends_with('"') && text.len() >= 2 {
                    &text[1..text.len() - 1]
                } else {
                    text
                };
                script
                    .commands
                    .push(TapeCommand::Type(stripped.to_string()));
            }
            "enter" => {
                script.commands.push(TapeCommand::Enter);
            }
            "space" => {
                let count = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(1);
                script.commands.push(TapeCommand::Space(count));
            }
            "backspace" => {
                let count = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(1);
                script.commands.push(TapeCommand::Backspace(count));
            }
            "sleep" => {
                if parts.len() < 2 {
                    return Err(format!(
                        "Line {}: Sleep requires duration (e.g. 500ms, 2s)",
                        line_num
                    ));
                }
                let dur = parse_duration(parts[1]).ok_or_else(|| {
                    format!("Line {}: Invalid duration format '{}'", line_num, parts[1])
                })?;
                script.commands.push(TapeCommand::Sleep(dur));
            }
            "ctrl+c" | "ctrl-c" => script.commands.push(TapeCommand::Ctrl('c')),
            "ctrl+d" | "ctrl-d" => script.commands.push(TapeCommand::Ctrl('d')),
            "ctrl+l" | "ctrl-l" => script.commands.push(TapeCommand::Ctrl('l')),
            _ => {
                return Err(format!(
                    "Line {}: Unknown instruction '{}'",
                    line_num, keyword
                ));
            }
        }
    }

    Ok(script)
}

fn parse_duration(s: &str) -> Option<Duration> {
    let s = s.trim().to_ascii_lowercase();
    if let Some(rest) = s.strip_suffix("ms") {
        let ms: u64 = rest.trim().parse().ok()?;
        Some(Duration::from_millis(ms))
    } else if let Some(rest) = s.strip_suffix('s') {
        let secs: f64 = rest.trim().parse().ok()?;
        Some(Duration::from_secs_f64(secs))
    } else {
        let secs: f64 = s.parse().ok()?;
        Some(Duration::from_secs_f64(secs))
    }
}
