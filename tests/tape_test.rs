use std::path::PathBuf;
use std::time::Duration;
use termloom::tape::parser::{parse_tape, TapeCommand};

#[test]
fn test_tape_parser_basic() {
    let script_content = r#"
# Demo Tape Script
Output demo.svg
Set FontSize 16
Set TypingSpeed 50ms
Set Width 90
Set Height 25

Type "echo 'TermLoom Automation'"
Enter
Sleep 1s
Type "git status"
Enter
Sleep 500ms
Ctrl+C
"#;

    let script = parse_tape(script_content).expect("Failed to parse tape");
    assert_eq!(script.output, Some(PathBuf::from("demo.svg")));
    assert_eq!(script.font_size, 16);
    assert_eq!(script.typing_speed, Duration::from_millis(50));
    assert_eq!(script.cols, 90);
    assert_eq!(script.rows, 25);

    assert_eq!(script.commands.len(), 7);
    assert_eq!(
        script.commands[0],
        TapeCommand::Type("echo 'TermLoom Automation'".to_string())
    );
    assert_eq!(script.commands[1], TapeCommand::Enter);
    assert_eq!(
        script.commands[2],
        TapeCommand::Sleep(Duration::from_millis(1000))
    );
    assert_eq!(
        script.commands[3],
        TapeCommand::Type("git status".to_string())
    );
    assert_eq!(script.commands[4], TapeCommand::Enter);
    assert_eq!(
        script.commands[5],
        TapeCommand::Sleep(Duration::from_millis(500))
    );
    assert_eq!(script.commands[6], TapeCommand::Ctrl('c'));
}
