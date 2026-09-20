//! Asciinema v2 (.cast) file serialization and deserialization.

use super::event::RawEvent;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{BufRead, Write};

#[derive(Serialize, Deserialize, Debug)]
pub struct CastHeader {
    pub version: u32,
    pub width: usize,
    pub height: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,
}

/// Write raw events to an Asciinema v2 format writer.
pub fn write_cast<W: Write>(
    writer: &mut W,
    events: &[RawEvent],
    cols: usize,
    rows: usize,
    title: &str,
) -> std::io::Result<()> {
    let now = chrono::Utc::now().timestamp() as u64;
    let header = CastHeader {
        version: 2,
        width: cols,
        height: rows,
        timestamp: Some(now),
        title: Some(title.to_string()),
        env: None,
    };

    let header_json = serde_json::to_string(&header)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    writeln!(writer, "{}", header_json)?;

    for event in events {
        // Asciinema v2 stores text in JSON string
        let text = String::from_utf8_lossy(&event.data);
        let tuple = (event.timestamp, "o", text);
        let line = serde_json::to_string(&tuple)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        writeln!(writer, "{}", line)?;
    }

    Ok(())
}

/// Read an Asciinema v2 format stream into dimensions and raw events.
pub fn read_cast<R: BufRead>(
    mut reader: R,
) -> Result<(usize, usize, Vec<RawEvent>), Box<dyn std::error::Error + Send + Sync>> {
    let mut first_line = String::new();
    reader.read_line(&mut first_line)?;

    let header: CastHeader = serde_json::from_str(&first_line)?;
    if header.version != 2 {
        return Err(format!("Unsupported asciinema version: {}", header.version).into());
    }

    let mut events = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Asciinema record: [time, type, data]
        if let Ok(record) = serde_json::from_str::<serde_json::Value>(trimmed) {
            if let Some(arr) = record.as_array() {
                if arr.len() >= 3 {
                    let time = arr[0].as_f64().unwrap_or(0.0);
                    let event_type = arr[1].as_str().unwrap_or("");
                    let data = arr[2].as_str().unwrap_or("");

                    if event_type == "o" {
                        events.push(RawEvent::new(time, data.as_bytes().to_vec()));
                    }
                }
            }
        }
    }

    Ok((header.width, header.height, events))
}
