use termloom::emulator::grid::ScreenGrid;
use termloom::recorder::compressor::Compressor;
use termloom::recorder::event::RawEvent;
use termloom::recorder::frame::compute_grid_hash;

#[test]
fn test_hash_sensitivity_on_cell_change() {
    let mut grid1 = ScreenGrid::new(40, 10);
    let mut grid2 = ScreenGrid::new(40, 10);

    let h1 = compute_grid_hash(&grid1);
    let h2 = compute_grid_hash(&grid2);
    assert_eq!(h1, h2);

    // Modify a single character
    grid2.cells[0][0].grapheme = "A".to_string();
    let h3 = compute_grid_hash(&grid2);
    assert_ne!(h1, h3);

    // Modify cursor
    grid1.cursor_col = 5;
    let h4 = compute_grid_hash(&grid1);
    assert_ne!(h1, h4);
}

#[test]
fn test_consecutive_identical_frames_deduplication() {
    let compressor = Compressor::new(40, 10, 2.0);

    // Send multiple events where text doesn't change
    let events = vec![
        RawEvent::new(0.0, b"Hello".to_vec()),
        RawEvent::new(0.5, b"".to_vec()),
        RawEvent::new(1.0, b"".to_vec()),
        RawEvent::new(1.5, b"".to_vec()),
    ];

    let frames = compressor.process(&events);

    // All empty events after "Hello" should have merged into 1 frame
    assert_eq!(frames.len(), 1);
    // Duration should be clamped or combined
    assert!(frames[0].duration > 0.0);
}

#[test]
fn test_idle_interval_clamping() {
    let max_wait = 1.2;
    let compressor = Compressor::new(40, 10, max_wait);

    // Initial event, then user types "A", waits 15 seconds, types "B"
    let events = vec![
        RawEvent::new(0.0, b"Start\r\n".to_vec()),
        RawEvent::new(0.2, b"A".to_vec()),
        RawEvent::new(15.2, b"B".to_vec()),
    ];

    let frames = compressor.process(&events);
    assert_eq!(frames.len(), 3);

    // Frame 1 should have been clamped from 15.0s to 1.2s max_wait
    assert!((frames[1].duration - max_wait).abs() < 0.001);
}
