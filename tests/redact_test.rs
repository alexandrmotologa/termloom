use termloom::emulator::grid::ScreenGrid;
use termloom::recorder::redact::Redactor;

fn grid_from_text(text: &str, cols: usize, rows: usize) -> ScreenGrid {
    let mut grid = ScreenGrid::new(cols, rows);
    for (r, line) in text.lines().enumerate() {
        if r >= rows {
            break;
        }
        for (c, ch) in line.chars().enumerate() {
            if c >= cols {
                break;
            }
            grid.cells[r][c].grapheme = ch.to_string();
        }
    }
    grid
}

fn grid_to_line(grid: &ScreenGrid, row: usize) -> String {
    grid.cells[row]
        .iter()
        .map(|c| c.grapheme.as_str())
        .collect::<Vec<_>>()
        .join("")
        .trim_end()
        .to_string()
}

#[test]
fn test_default_secret_masking() {
    let redactor = Redactor::standard();

    // Construct test tokens at runtime to verify redaction without triggering repository secret scanners
    let mock_gh_token = format!("{}_{}", "ghp", "0123456789abcdefghijklmnopqrstuvwxyz01");
    let mut grid = grid_from_text(&mock_gh_token, 50, 2);
    let count = redactor.redact_grid(&mut grid);
    assert_eq!(count, 1);
    let line = grid_to_line(&grid, 0);
    assert_eq!(line, "******************************************");

    let mock_aws_key = format!("{}{}", "AKIA", "0123456789ABCDEF");
    let mut aws_grid = grid_from_text(&mock_aws_key, 30, 2);
    let aws_count = redactor.redact_grid(&mut aws_grid);
    assert_eq!(aws_count, 1);
    assert_eq!(grid_to_line(&aws_grid, 0), "********************");
}

#[test]
fn test_custom_regex_masking() {
    let redactor = Redactor::with_custom(&["mycorp_token_[a-z0-9]+".to_string()])
        .expect("Failed to create custom redactor");

    let mut grid = grid_from_text("export SESSION=mycorp_token_xyz987654", 60, 2);
    let count = redactor.redact_grid(&mut grid);
    assert_eq!(count, 1);
    assert_eq!(
        grid_to_line(&grid, 0),
        "export SESSION=**********************"
    );
}

#[test]
fn test_clean_text_untouched() {
    let redactor = Redactor::standard();
    let text = "cargo test --all";
    let mut grid = grid_from_text(text, 30, 2);
    let count = redactor.redact_grid(&mut grid);
    assert_eq!(count, 0);
    assert_eq!(grid_to_line(&grid, 0), text);
}
