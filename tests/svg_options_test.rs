use termloom::cli::WindowStyle;
use termloom::emulator::grid::ScreenGrid;
use termloom::exporters::svg::SvgRenderer;
use termloom::recorder::frame::Frame;
use termloom::themes::CATPPUCCIN_MOCHA;

#[test]
fn test_svg_advanced_options_rendering() {
    let mut grid = ScreenGrid::new(40, 10);
    grid.cells[0][0].grapheme = "H".to_string();

    let frames = vec![Frame::new(grid, 1.0, 0.0)];

    let renderer = SvgRenderer::new(
        &CATPPUCCIN_MOCHA,
        WindowStyle::Macos,
        "termloom-options-test",
        "monospace",
        14,
        1.35,
    )
    .with_hover_pause(true)
    .with_font_url(Some("https://fonts.googleapis.com/css2?family=Fira+Code"))
    .with_shadow(true);

    let svg = renderer.render(&frames, 40, 10);

    assert!(svg.contains("animation-play-state: paused !important"));
    assert!(svg.contains("@import url('https://fonts.googleapis.com/css2?family=Fira+Code');"));
    assert!(svg.contains("filter: drop-shadow"));
    assert!(svg.contains("class=\"window-frame\""));
}
