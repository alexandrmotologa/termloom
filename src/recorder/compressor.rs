//! Frame compression, xxHash deduplication, and idle time clamping.

use super::event::RawEvent;
use super::frame::Frame;
use crate::emulator::handler::VteHandler;
use vte::Parser;

pub struct Compressor {
    cols: usize,
    rows: usize,
    max_wait: f64,
}

impl Compressor {
    pub fn new(cols: usize, rows: usize, max_wait: f64) -> Self {
        Self {
            cols,
            rows,
            max_wait: max_wait.max(0.1),
        }
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

        // Step 3: Clamp idle intervals and trim leading latency
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
