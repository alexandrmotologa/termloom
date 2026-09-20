//! Terminal raw mode RAII guard.

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::sync::atomic::{AtomicBool, Ordering};

static RAW_MODE_ACTIVE: AtomicBool = AtomicBool::new(false);

/// RAII guard that manages host terminal raw mode.
pub struct RawModeGuard {
    active: bool,
}

impl RawModeGuard {
    pub fn enter() -> std::io::Result<Self> {
        enable_raw_mode()?;
        RAW_MODE_ACTIVE.store(true, Ordering::SeqCst);
        Ok(Self { active: true })
    }

    pub fn restore(&mut self) {
        if self.active {
            let _ = disable_raw_mode();
            RAW_MODE_ACTIVE.store(false, Ordering::SeqCst);
            self.active = false;
        }
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        self.restore();
    }
}
