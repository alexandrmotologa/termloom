//! Standalone interactive HTML player exporter.

use crate::emulator::color::Color;
use crate::recorder::frame::Frame;
use crate::themes::Palette;
use serde::Serialize;

#[derive(Serialize)]
struct HtmlFrameData {
    time: f64,
    dur: f64,
    cursor: (usize, usize, bool),
    lines: Vec<Vec<HtmlRun>>,
}

#[derive(Serialize)]
struct HtmlRun {
    text: String,
    fg: String,
    bg: Option<String>,
    bold: bool,
    italic: bool,
    under: bool,
    strike: bool,
    link: Option<String>,
}

pub struct HtmlRenderer<'a> {
    palette: &'a Palette,
    title: &'a str,
}

impl<'a> HtmlRenderer<'a> {
    pub fn new(palette: &'a Palette, title: &'a str) -> Self {
        Self { palette, title }
    }

    pub fn render(&self, frames: &[Frame], _cols: usize, rows: usize) -> String {
        let total_duration: f64 = frames.iter().map(|f| f.duration).sum::<f64>().max(0.1);

        // Extract any typed command from prompt lines
        let detected_command = extract_command_from_frames(frames);

        // Serialize frame data
        let mut frame_data = Vec::with_capacity(frames.len());
        for frame in frames {
            let mut lines = Vec::with_capacity(frame.grid.rows);
            for row in &frame.grid.cells {
                let mut runs = Vec::new();
                for cell in row {
                    if cell.is_continuation {
                        continue;
                    }
                    let effective_fg = if cell.inverse { cell.bg } else { cell.fg };
                    let effective_bg = if cell.inverse { cell.fg } else { cell.bg };

                    let bg_str = if effective_bg != Color::DefaultBg {
                        Some(effective_bg.to_hex(self.palette, false))
                    } else {
                        None
                    };

                    runs.push(HtmlRun {
                        text: cell.grapheme.clone(),
                        fg: effective_fg.to_hex(self.palette, true),
                        bg: bg_str,
                        bold: cell.bold,
                        italic: cell.italic,
                        under: cell.underline,
                        strike: cell.strikethrough,
                        link: cell.link.clone(),
                    });
                }
                // Trim trailing blanks
                while let Some(last) = runs.last() {
                    if last.text.trim().is_empty() && last.bg.is_none() && last.link.is_none() {
                        runs.pop();
                    } else {
                        break;
                    }
                }
                lines.push(runs);
            }

            frame_data.push(HtmlFrameData {
                time: frame.timestamp,
                dur: frame.duration,
                cursor: (
                    frame.grid.cursor_col,
                    frame.grid.cursor_row,
                    frame.grid.cursor_visible,
                ),
                lines,
            });
        }

        let frames_json = serde_json::to_string(&frame_data).unwrap_or_else(|_| "[]".to_string());
        let bg_hex = Palette::hex(self.palette.bg);
        let fg_hex = Palette::hex(self.palette.fg);
        let cursor_hex = Palette::hex(self.palette.cursor);

        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>{title}</title>
  <style>
    * {{ box-sizing: border-box; margin: 0; padding: 0; }}
    body {{
      background: #090d16;
      color: #e2e8f0;
      font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      min-height: 100vh;
      padding: 24px;
    }}
    .player-card {{
      background: {bg};
      border: 1px solid #334155;
      border-radius: 12px;
      box-shadow: 0 20px 40px -15px rgba(0,0,0,0.6);
      width: 100%;
      max-width: 960px;
      overflow: hidden;
      display: flex;
      flex-direction: column;
    }}
    .window-header {{
      background: rgba(15, 23, 42, 0.6);
      border-bottom: 1px solid #334155;
      padding: 10px 16px;
      display: flex;
      align-items: center;
      justify-content: space-between;
      user-select: none;
    }}
    .window-buttons {{
      display: flex;
      gap: 8px;
    }}
    .window-dot {{
      width: 12px;
      height: 12px;
      border-radius: 50%;
    }}
    .dot-red {{ background: #ef4444; }}
    .dot-yellow {{ background: #f59e0b; }}
    .dot-green {{ background: #10b981; }}
    .window-title {{
      font-size: 13px;
      font-weight: 600;
      color: #94a3b8;
      letter-spacing: 0.5px;
    }}
    .header-actions {{
      display: flex;
      gap: 8px;
    }}
    .copy-btn {{
      background: #1e293b;
      border: 1px solid #475569;
      color: #cbd5e1;
      padding: 4px 10px;
      border-radius: 6px;
      font-size: 12px;
      font-weight: 500;
      cursor: pointer;
      display: flex;
      align-items: center;
      gap: 6px;
      transition: all 0.15s ease;
    }}
    .copy-btn:hover {{
      background: #334155;
      color: #fff;
    }}
    .terminal-view {{
      padding: 16px;
      font-family: "JetBrains Mono", "Fira Code", "Cascadia Code", "SF Mono", Menlo, Consolas, monospace;
      font-size: 14px;
      line-height: 1.35;
      min-height: 400px;
      overflow-x: auto;
      white-space: pre;
      color: {fg};
      position: relative;
    }}
    .cursor {{
      display: inline-block;
      width: 8.5px;
      height: 16px;
      background: {cursor};
      vertical-align: text-bottom;
      opacity: 0.8;
    }}
    .controls-bar {{
      background: rgba(15, 23, 42, 0.8);
      border-top: 1px solid #334155;
      padding: 10px 16px;
      display: flex;
      align-items: center;
      gap: 14px;
      user-select: none;
    }}
    .play-btn {{
      background: transparent;
      border: none;
      color: #38bdf8;
      font-size: 18px;
      cursor: pointer;
      width: 28px;
      height: 28px;
      display: flex;
      align-items: center;
      justify-content: center;
      border-radius: 4px;
      transition: background 0.15s;
    }}
    .play-btn:hover {{ background: #1e293b; }}
    .time-slider {{
      flex: 1;
      accent-color: #38bdf8;
      cursor: pointer;
    }}
    .time-label {{
      font-size: 12px;
      color: #94a3b8;
      font-variant-numeric: tabular-nums;
      min-width: 90px;
      text-align: right;
    }}
    .speed-btn {{
      background: #1e293b;
      border: 1px solid #334155;
      color: #94a3b8;
      font-size: 11px;
      padding: 3px 8px;
      border-radius: 4px;
      cursor: pointer;
    }}
    .speed-btn:hover {{
      color: #fff;
      border-color: #475569;
    }}
  </style>
</head>
<body>
  <div class="player-card">
    <div class="window-header">
      <div class="window-buttons">
        <div class="window-dot dot-red"></div>
        <div class="window-dot dot-yellow"></div>
        <div class="window-dot dot-green"></div>
      </div>
      <div class="window-title">{title}</div>
      <div class="header-actions">
        <button class="copy-btn" id="copyBtn" onclick="copyCommand()">
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg>
          <span id="copyBtnText">Copy Command</span>
        </button>
      </div>
    </div>
    <div class="terminal-view" id="termView"></div>
    <div class="controls-bar">
      <button class="play-btn" id="playBtn" onclick="togglePlay()">▶</button>
      <input type="range" class="time-slider" id="timeSlider" min="0" max="{total_duration:.3}" step="0.05" value="0" oninput="onSeek(this.value)">
      <div class="time-label" id="timeLabel">0:00 / {total_formatted}</div>
      <button class="speed-btn" id="speedBtn" onclick="cycleSpeed()">1.0x</button>
    </div>
  </div>

  <script>
    const frames = {frames_json};
    const totalDuration = {total_duration:.3};
    const detectedCommand = "{detected_cmd}";
    let currentIndex = 0;
    let isPlaying = true;
    let playSpeed = 1.0;
    let timer = null;
    let currentSimTime = 0.0;

    const termView = document.getElementById("termView");
    const playBtn = document.getElementById("playBtn");
    const timeSlider = document.getElementById("timeSlider");
    const timeLabel = document.getElementById("timeLabel");
    const speedBtn = document.getElementById("speedBtn");
    const copyBtnText = document.getElementById("copyBtnText");

    function formatTime(secs) {{
      const m = Math.floor(secs / 60);
      const s = Math.floor(secs % 60);
      return m + ":" + (s < 10 ? "0" : "") + s;
    }}

    function renderFrame(idx) {{
      if (idx < 0 || idx >= frames.length) return;
      const f = frames[idx];
      let html = "";
      for (let r = 0; r < {rows}; r++) {{
        const line = f.lines[r] || [];
        for (const run of line) {{
          let style = "color:" + run.fg + ";";
          if (run.bg) style += "background-color:" + run.bg + ";";
          if (run.bold) style += "font-weight:bold;";
          if (run.italic) style += "font-style:italic;";
          if (run.under && run.strike) style += "text-decoration:underline line-through;";
          else if (run.under) style += "text-decoration:underline;";
          else if (run.strike) style += "text-decoration:line-through;";

          const text = run.text.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
          if (run.link) {{
            html += '<a href="' + run.link + '" target="_blank" style="' + style + '">' + text + '</a>';
          }} else {{
            html += '<span style="' + style + '">' + text + '</span>';
          }}
        }}
        html += "\n";
      }}
      termView.innerHTML = html;
      timeSlider.value = f.time;
      timeLabel.textContent = formatTime(f.time) + " / " + formatTime(totalDuration);
    }}

    function tick() {{
      if (!isPlaying) return;
      currentSimTime += 0.05 * playSpeed;
      if (currentSimTime > totalDuration) {{
        currentSimTime = 0.0;
      }}
      // Find frame
      let matchIdx = 0;
      for (let i = 0; i < frames.length; i++) {{
        if (frames[i].time <= currentSimTime) {{
          matchIdx = i;
        }} else {{
          break;
        }}
      }}
      if (matchIdx !== currentIndex) {{
        currentIndex = matchIdx;
        renderFrame(currentIndex);
      }}
    }}

    function togglePlay() {{
      isPlaying = !isPlaying;
      playBtn.textContent = isPlaying ? "⏸" : "▶";
    }}

    function onSeek(val) {{
      currentSimTime = parseFloat(val);
      let matchIdx = 0;
      for (let i = 0; i < frames.length; i++) {{
        if (frames[i].time <= currentSimTime) matchIdx = i;
        else break;
      }}
      currentIndex = matchIdx;
      renderFrame(currentIndex);
    }}

    const speeds = [0.5, 1.0, 1.5, 2.0];
    let speedIdx = 1;
    function cycleSpeed() {{
      speedIdx = (speedIdx + 1) % speeds.length;
      playSpeed = speeds[speedIdx];
      speedBtn.textContent = playSpeed.toFixed(1) + "x";
    }}

    function copyCommand() {{
      const cmd = detectedCommand || "termloom record";
      navigator.clipboard.writeText(cmd).then(() => {{
        copyBtnText.textContent = "Copied!";
        setTimeout(() => {{ copyBtnText.textContent = "Copy Command"; }}, 2000);
      }});
    }}

    // Initial render
    renderFrame(0);
    setInterval(tick, 50);
  </script>
</body>
</html>
"#,
            title = self.title,
            bg = bg_hex,
            fg = fg_hex,
            cursor = cursor_hex,
            total_duration = total_duration,
            total_formatted = format_seconds(total_duration),
            rows = rows,
            frames_json = frames_json,
            detected_cmd = detected_command.replace('"', "\\\""),
        )
    }
}

fn format_seconds(secs: f64) -> String {
    let m = (secs / 60.0).floor() as u32;
    let s = (secs % 60.0).floor() as u32;
    format!("{}:{:02}", m, s)
}

fn extract_command_from_frames(frames: &[Frame]) -> String {
    // Scan for lines containing prompt characters and extract following text
    for frame in frames.iter().rev() {
        for row in &frame.grid.cells {
            let line_str: String = row.iter().map(|c| c.grapheme.as_str()).collect();
            let trimmed = line_str.trim();

            if let Some(pos) = trimmed.find('$') {
                let cmd = trimmed[pos + 1..].trim();
                if !cmd.is_empty() {
                    return cmd.to_string();
                }
            } else if let Some(pos) = trimmed.find('>') {
                let cmd = trimmed[pos + 1..].trim();
                if !cmd.is_empty() {
                    return cmd.to_string();
                }
            } else if let Some(pos) = trimmed.find('#') {
                let cmd = trimmed[pos + 1..].trim();
                if !cmd.is_empty() {
                    return cmd.to_string();
                }
            }
        }
    }
    String::new()
}
