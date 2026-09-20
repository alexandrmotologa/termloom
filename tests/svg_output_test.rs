use termloom::cli::WindowStyle;
use termloom::emulator::grid::ScreenGrid;
use termloom::exporters::svg::SvgRenderer;
use termloom::recorder::frame::Frame;
use termloom::themes::CATPPUCCIN_MOCHA;

#[test]
fn test_svg_basic_structure() {
    let mut grid = ScreenGrid::new(40, 10);
    grid.cells[0][0].grapheme = "H".to_string();
    grid.cells[0][1].grapheme = "i".to_string();

    let frames = vec![Frame::new(grid, 1.5, 0.0)];

    let renderer = SvgRenderer::new(
        &CATPPUCCIN_MOCHA,
        WindowStyle::Macos,
        "termloom-test",
        "monospace",
        14,
        1.35,
    );

    let svg = renderer.render(&frames, 40, 10);

    assert!(svg.starts_with("<svg"));
    assert!(svg.ends_with("</svg>\n"));
    assert!(svg.contains("xmlns=\"http://www.w3.org/2000/svg\""));
    assert!(svg.contains("termloom-test"));
    assert!(svg.contains("@keyframes f_0"));
    assert!(svg.contains("<circle cx=\"20\" cy=\"19\" r=\"6\" fill=\"#ef4444\""));
}

#[test]
fn test_svg_xml_escaping() {
    let mut grid = ScreenGrid::new(40, 10);
    // Write characters requiring XML escaping
    grid.cells[0][0].grapheme = "<".to_string();
    grid.cells[0][1].grapheme = "&".to_string();
    grid.cells[0][2].grapheme = ">".to_string();

    let frames = vec![Frame::new(grid, 1.0, 0.0)];

    let renderer = SvgRenderer::new(
        &CATPPUCCIN_MOCHA,
        WindowStyle::None,
        "test",
        "monospace",
        14,
        1.35,
    );

    let svg = renderer.render(&frames, 40, 10);

    assert!(svg.contains("&lt;&amp;&gt;"));
    assert!(!svg.contains("<text x=\"8.0\" y=\"19.5\"><tspan fill=\"#cdd6f4\"><&></tspan>"));
}

#[test]
fn test_svg_hyperlink_rendering() {
    let mut grid = ScreenGrid::new(40, 10);
    grid.cells[0][0].grapheme = "U".to_string();
    grid.cells[0][0].link = Some("https://example.com".to_string());

    let frames = vec![Frame::new(grid, 1.0, 0.0)];

    let renderer = SvgRenderer::new(
        &CATPPUCCIN_MOCHA,
        WindowStyle::Squircle,
        "test",
        "monospace",
        14,
        1.35,
    );

    let svg = renderer.render(&frames, 40, 10);

    assert!(svg.contains(r#"<a xlink:href="https://example.com""#));
}
