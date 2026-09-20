//! Cross-platform PTY session lifecycle management.

use super::raw_mode::RawModeGuard;
use crate::recorder::event::RawEvent;
use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use std::io::{Read, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

pub fn detect_shell() -> (String, Vec<String>) {
    #[cfg(windows)]
    {
        if let Ok(pwsh) = std::env::var("COMSPEC") {
            // Check for PowerShell or use COMSPEC
            if which_exists("pwsh.exe") {
                return ("pwsh.exe".to_string(), vec!["-NoLogo".to_string()]);
            }
            if which_exists("powershell.exe") {
                return ("powershell.exe".to_string(), vec!["-NoLogo".to_string()]);
            }
            (pwsh, vec![])
        } else {
            ("cmd.exe".to_string(), vec![])
        }
    }

    #[cfg(not(windows))]
    {
        if let Ok(shell) = std::env::var("SHELL") {
            (shell, vec![])
        } else {
            ("/bin/sh".to_string(), vec![])
        }
    }
}

fn which_exists(prog: &str) -> bool {
    if let Ok(path) = std::env::var("PATH") {
        for dir in std::env::split_paths(&path) {
            let full = dir.join(prog);
            if full.is_file() {
                return true;
            }
        }
    }
    false
}

pub fn run_session(
    custom_command: Option<&str>,
    cols: u16,
    rows: u16,
) -> Result<Vec<RawEvent>, Box<dyn std::error::Error + Send + Sync>> {
    let pty_system = native_pty_system();
    let pair = pty_system.openpty(PtySize {
        rows,
        cols,
        pixel_width: 0,
        pixel_height: 0,
    })?;

    let cmd = if let Some(custom) = custom_command {
        #[cfg(windows)]
        {
            let mut c = CommandBuilder::new("cmd.exe");
            c.args(["/C", custom]);
            c
        }
        #[cfg(not(windows))]
        {
            let mut c = CommandBuilder::new("sh");
            c.args(["-c", custom]);
            c
        }
    } else {
        let (shell, args) = detect_shell();
        let mut c = CommandBuilder::new(shell);
        c.args(args);
        c
    };

    let mut child = pair.slave.spawn_command(cmd)?;
    // Slave handle must be dropped in the parent so EOF is triggered when child exits
    drop(pair.slave);

    let mut reader = pair.master.try_clone_reader()?;
    let mut writer = pair.master.take_writer()?;

    let running = Arc::new(AtomicBool::new(true));
    let is_interactive = custom_command.is_none();

    // Enable host raw mode for interactive sessions
    let _raw_guard = if is_interactive {
        match RawModeGuard::enter() {
            Ok(g) => Some(g),
            Err(e) => {
                eprintln!("[termloom] Note: Failed to enable raw mode on host: {}", e);
                None
            }
        }
    } else {
        None
    };

    // Stdin forwarder thread for interactive sessions
    if is_interactive {
        let running_clone = Arc::clone(&running);
        std::thread::spawn(move || {
            let mut stdin = std::io::stdin();
            let mut buf = [0u8; 1024];
            while running_clone.load(Ordering::Relaxed) {
                match stdin.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        if writer.write_all(&buf[..n]).is_err() {
                            break;
                        }
                        let _ = writer.flush();
                    }
                    Err(_) => break,
                }
            }
        });
    }

    let (tx, rx) = std::sync::mpsc::channel();
    let reader_running = Arc::clone(&running);

    std::thread::spawn(move || {
        let mut buf = [0u8; 4096];
        while reader_running.load(Ordering::Relaxed) {
            match reader.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    if tx.send(buf[..n].to_vec()).is_err() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    });

    let mut events = Vec::new();
    let start_time = Instant::now();
    let mut last_title_update = Instant::now();
    let mut stdout = std::io::stdout();

    // Main event loop: captures PTY output and detects process termination
    loop {
        if is_interactive && last_title_update.elapsed() >= Duration::from_secs(1) {
            let elapsed_secs = start_time.elapsed().as_secs();
            let title_str = format!(
                "\x1b]0;🔴 [REC] TermLoom ({:02}:{:02}) - Type 'exit' to finish\x07",
                elapsed_secs / 60,
                elapsed_secs % 60
            );
            let _ = stdout.write_all(title_str.as_bytes());
            let _ = stdout.flush();
            last_title_update = Instant::now();
        }

        match rx.recv_timeout(std::time::Duration::from_millis(30)) {
            Ok(chunk) => {
                let elapsed = start_time.elapsed().as_secs_f64();
                events.push(RawEvent::new(elapsed, chunk.clone()));

                // Mirror to user's terminal
                let _ = stdout.write_all(&chunk);
                let _ = stdout.flush();
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                // Check if child process has terminated
                if let Ok(Some(_)) = child.try_wait() {
                    // Small grace period to drain remaining bytes
                    std::thread::sleep(std::time::Duration::from_millis(50));
                    while let Ok(chunk) = rx.try_recv() {
                        let elapsed = start_time.elapsed().as_secs_f64();
                        events.push(RawEvent::new(elapsed, chunk.clone()));
                        let _ = stdout.write_all(&chunk);
                        let _ = stdout.flush();
                    }
                    break;
                }
            }
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                break;
            }
        }
    }

    if is_interactive {
        // Reset terminal window title
        let _ = stdout.write_all(b"\x1b]0;\x07");
        let _ = stdout.flush();
    }

    running.store(false, Ordering::Relaxed);
    let _ = child.wait();

    Ok(events)
}
