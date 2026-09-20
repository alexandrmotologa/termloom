use termloom::recorder::compressor::Compressor;
use termloom::recorder::event::RawEvent;

#[test]
fn test_trim_exit_removes_trailing_exit_command() {
    let events = vec![
        RawEvent::new(0.1, b"prompt$ ls\nfile1 file2\n".to_vec()),
        RawEvent::new(0.5, b"prompt$ exit\r\n".to_vec()),
    ];

    let frames_without_trim = Compressor::new(80, 24, 2.0)
        .with_trim_exit(false)
        .process(&events);
    assert_eq!(frames_without_trim.len(), 2);

    let frames_with_trim = Compressor::new(80, 24, 2.0)
        .with_trim_exit(true)
        .process(&events);
    assert_eq!(frames_with_trim.len(), 1);
}
