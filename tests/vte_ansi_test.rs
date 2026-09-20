use termloom::emulator::color::Color;
use termloom::emulator::handler::VteHandler;
use vte::Parser;

#[test]
fn test_truecolor_sgr_parsing() {
    let mut parser = Parser::new();
    let mut handler = VteHandler::new(80, 24);

    // 24-bit TrueColor foreground: RGB(123, 234, 45)
    let seq = b"\x1b[38;2;123;234;45mHello\x1b[0m";
    parser.advance(&mut handler, seq);

    let cell = &handler.grid.cells[0][0];
    assert_eq!(cell.grapheme, "H");
    assert_eq!(cell.fg, Color::Rgb(123, 234, 45));

    // Reset style
    assert_eq!(handler.style.fg, Color::DefaultFg);
}

#[test]
fn test_ansi_256_color_parsing() {
    let mut parser = Parser::new();
    let mut handler = VteHandler::new(80, 24);

    // ANSI 256 color foreground: index 196
    let seq = b"\x1b[38;5;196mRed\x1b[0m";
    parser.advance(&mut handler, seq);

    let cell = &handler.grid.cells[0][0];
    assert_eq!(cell.grapheme, "R");
    assert_eq!(cell.fg, Color::Indexed(196));
}

#[test]
fn test_sgr_styles_bold_italic_underline() {
    let mut parser = Parser::new();
    let mut handler = VteHandler::new(80, 24);

    // Bold (1), Italic (3), Underline (4)
    let seq = b"\x1b[1;3;4mStyled\x1b[0m";
    parser.advance(&mut handler, seq);

    let cell = &handler.grid.cells[0][0];
    assert_eq!(cell.grapheme, "S");
    assert!(cell.bold);
    assert!(cell.italic);
    assert!(cell.underline);
}

#[test]
fn test_cursor_movement_and_clear() {
    let mut parser = Parser::new();
    let mut handler = VteHandler::new(80, 24);

    // Write at line 1, move to row 5 col 10, write 'X'
    let seq = b"Line 1\x1b[5;10HX";
    parser.advance(&mut handler, seq);

    assert_eq!(handler.grid.cells[0][0].grapheme, "L");
    assert_eq!(handler.grid.cells[4][9].grapheme, "X");

    // Clear line 5
    parser.advance(&mut handler, b"\x1b[2K");
    assert_eq!(handler.grid.cells[4][9].grapheme, " ");
}

#[test]
fn test_osc8_hyperlink_parsing() {
    let mut parser = Parser::new();
    let mut handler = VteHandler::new(80, 24);

    // OSC 8 hyperlink: \x1b]8;;https://termloom.dev\x1b\Link\x1b]8;;\x1b\
    let seq = b"\x1b]8;;https://termloom.dev\x1b\\Link\x1b]8;;\x1b\\";
    parser.advance(&mut handler, seq);

    let cell = &handler.grid.cells[0][0];
    assert_eq!(cell.grapheme, "L");
    assert_eq!(cell.link.as_deref(), Some("https://termloom.dev"));

    // After closing OSC 8, link should be reset
    assert_eq!(handler.style.link, None);
}

#[test]
fn test_unicode_wide_character_handling() {
    let mut parser = Parser::new();
    let mut handler = VteHandler::new(80, 24);

    // Write Crab emoji (display width 2)
    let seq = "🦀".as_bytes();
    parser.advance(&mut handler, seq);

    let cell_0 = &handler.grid.cells[0][0];
    let cell_1 = &handler.grid.cells[0][1];

    assert_eq!(cell_0.grapheme, "🦀");
    assert_eq!(cell_0.width, 2);
    assert!(!cell_0.is_continuation);

    assert!(cell_1.is_continuation);
    assert_eq!(cell_1.width, 0);

    // Cursor should have advanced by 2
    assert_eq!(handler.grid.cursor_col, 2);
}
