//! Raw timestamped PTY event chunk.

#[derive(Debug, Clone)]
pub struct RawEvent {
    /// Elapsed seconds since session start.
    pub timestamp: f64,
    /// Raw byte sequence emitted by the terminal PTY.
    pub data: Vec<u8>,
}

impl RawEvent {
    pub fn new(timestamp: f64, data: Vec<u8>) -> Self {
        Self { timestamp, data }
    }
}
