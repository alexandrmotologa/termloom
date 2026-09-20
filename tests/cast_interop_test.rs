use std::io::Cursor;
use termloom::recorder::cast::{read_cast, write_cast};
use termloom::recorder::event::RawEvent;

#[test]
fn test_cast_roundtrip_serialization() {
    let events = vec![
        RawEvent::new(0.0, b"echo 'hello'\r\n".to_vec()),
        RawEvent::new(0.5, b"hello\r\n".to_vec()),
        RawEvent::new(1.0, b"exit\r\n".to_vec()),
    ];

    let mut buffer = Vec::new();
    write_cast(&mut buffer, &events, 80, 24, "test-session").expect("Failed to write cast");

    let cursor = Cursor::new(buffer);
    let (cols, rows, read_events) = read_cast(cursor).expect("Failed to read cast");

    assert_eq!(cols, 80);
    assert_eq!(rows, 24);
    assert_eq!(read_events.len(), 3);
    assert_eq!(read_events[0].data, b"echo 'hello'\r\n");
    assert_eq!(read_events[1].data, b"hello\r\n");
    assert_eq!(read_events[2].data, b"exit\r\n");
}
