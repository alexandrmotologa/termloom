//! Frame compression, xxHash deduplication, secret redaction, and idle time clamping.

use super::event::RawEvent;
use super::frame::{compute_grid_hash, Frame};
use super::redact::Redactor;
use crate::emulator::handler::VteHandler;
use vte::Parser;

pub struct Compressor {
    cols: usize,
    rows: usize,
    max_wait: f64,
    redactor: Option<Redactor>,
    trim_exit: bool,
}

impl Compressor {
    pub fn new(cols: usize, rows: usize, max_wait: f64) -> Self {
        Self {
            cols,
            rows,
            max_wait: max_wait.max(0.1),
            redactor: None,
            trim_exit: false,
        }
    }

    pub fn with_redactor(mut self, redactor: Option<Redactor>) -> Self {
        self.redactor = redactor;
        self
    }

    pub fn with_trim_exit(mut self, trim_exit: bool) -> Self {
        self.trim_exit = trim_exit;
        self
    }

    /// Process a stream of raw events into an optimized list of deduplicated frames.
    pub fn process(&self, events: &[RawEvent]) -> Vec<Frame> {
        if events.is_empty() {
            let handler = VteHandler::new(self.cols, self.rows);
            return vec![Frame::new(handler.grid.clone(), 1.0, 0.0)];
        }

        let mut parser = Parser::new();
        let mut handler = VteHandler::new(self.cols, self.rows);

        // Step 1: Replay raw bytes and collect snapshots at event boundaries
        let mut raw_frames: Vec<(Frame, f64)> = Vec::new();

        for (i, event) in events.iter().enumerate() {
            parser.advance(&mut handler, &event.data);

            let next_timestamp = if i + 1 < events.len() {
                events[i + 1].timestamp
            } else {
                event.timestamp + 1.0 // Hold final frame for 1.0s
            };

            let duration = (next_timestamp - event.timestamp).max(0.01);
            let frame = Frame::new(handler.grid.clone(), duration, event.timestamp);
            raw_frames.push((frame, duration));
        }

        // Step 2: Deduplicate consecutive frames with identical visual hashes
        let mut deduplicated: Vec<Frame> = Vec::new();

        for (frame, raw_duration) in raw_frames {
            if let Some(prev) = deduplicated.last_mut() {
                if prev.hash == frame.hash {
                    prev.duration += raw_duration;
                    continue;
                }
            }
            deduplicated.push(frame);
        }

        // Step 3: Trim trailing 'exit' or 'logout' commands if enabled
        if self.trim_exit && deduplicated.len() > 1 {
            while deduplicated.len() > 1 {
                let last = deduplicated.last().unwrap();
                let last_line = last_non_empty_line(&last.grid);
                let trimmed = last_line.trim();

                let is_exit = trimmed.ends_with("exit")
                    || trimmed.ends_with("logout")
                    || trimmed.ends_with("quit");

                if is_exit {
                    deduplicated.pop();
                } else {
                    break;
                }
            }
        }

        // Step 4: Redact secrets across frames if enabled
        if let Some(ref redactor) = self.redactor {
            for frame in &mut deduplicated {
                redactor.redact_grid(&mut frame.grid);
                frame.hash = compute_grid_hash(&frame.grid);
            }
        }

        // Step 5: Clamp idle intervals and trim leading latency
        if let Some(first) = deduplicated.first_mut() {
            if first.duration > 0.5 {
                first.duration = 0.5;
            }
        }

        let mut cumulative_time = 0.0;
        for frame in &mut deduplicated {
            if frame.duration > self.max_wait {
                frame.duration = self.max_wait;
            }
            frame.timestamp = cumulative_time;
            cumulative_time += frame.duration;
        }

        deduplicated
    }
}

fn last_non_empty_line(grid: &crate::emulator::grid::ScreenGrid) -> String {
    for row in grid.cells.iter().rev() {
        let line: String = row.iter().map(|c| c.grapheme.as_str()).collect();
        if !line.trim().is_empty() {
            return line;
        }
    }
    String::new()
}
