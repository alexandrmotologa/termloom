//! Command line argument parsing and validation for TermLoom.

use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(
    name = "termloom",
    author = "Alexandr Motologa <alexandrmotologa@gmail.com>",
    version,
    about = "Cross-platform terminal session recorder and vector animator",
    long_about = "TermLoom records terminal streams directly from native PTYs (Windows ConPTY and POSIX openpty) and compiles them into pure vector animated SVGs, interactive standalone HTML players, and Asciinema v2 (.cast) files."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Record a terminal session to an animated SVG, HTML player, or Asciinema v2 file
    Record(RecordArgs),

    /// Convert an existing Asciinema v2 (.cast) recording into SVG or HTML
    Convert(ConvertArgs),

    /// Execute a scripted tape file to generate deterministic terminal recordings
    Run(RunArgs),

    /// Capture a single static terminal snapshot/screenshot with window chrome
    Snapshot(SnapshotArgs),
}

#[derive(Args, Debug, Clone)]
pub struct RecordArgs {
    /// Output file path (.svg, .html, or .cast)
    #[arg(default_value = "output.svg")]
    pub output: PathBuf,

    /// Specific command to execute instead of launching default interactive shell
    #[arg(short = 'c', long = "command")]
    pub command: Option<String>,

    /// Terminal column width
    #[arg(long, default_value_t = 100)]
    pub cols: u16,

    /// Terminal row count
    #[arg(long, default_value_t = 28)]
    pub rows: u16,

    /// Color theme name (catppuccin-mocha, catppuccin-latte, dracula, nord, tokyo-night, gruvbox-dark, solarized-dark, monokai, one-dark)
    #[arg(long, default_value = "catppuccin-mocha")]
    pub theme: String,

    /// Window frame style for SVG outputs
    #[arg(long, value_enum, default_value_t = WindowStyle::Macos)]
    pub window_style: WindowStyle,

    /// Maximum idle duration between frames in seconds
    #[arg(long, default_value_t = 1.5)]
    pub max_wait: f64,

    /// Output format override (svg, html, cast). If omitted, inferred from output extension
    #[arg(long, value_enum)]
    pub format: Option<OutputFormat>,

    /// Window title displayed in the frame header
    #[arg(long, default_value = "termloom")]
    pub title: String,

    /// CSS font family fallback sequence for SVG rendering
    #[arg(
        long,
        default_value = "'JetBrains Mono', 'Fira Code', 'Cascadia Code', 'SF Mono', Menlo, Consolas, monospace"
    )]
    pub font_family: String,

    /// Custom font URL to import in SVG (e.g. Google Fonts)
    #[arg(long)]
    pub font_url: Option<String>,

    /// Font size in pixels
    #[arg(long, default_value_t = 14)]
    pub font_size: u32,

    /// Line height multiplier for vertical alignment
    #[arg(long, default_value_t = 1.35)]
    pub line_height: f64,

    /// Automatically mask secrets (tokens, AWS keys, passwords)
    #[arg(long, default_value_t = false)]
    pub mask_secrets: bool,

    /// Custom regex patterns for secret redaction
    #[arg(long = "redact-regex")]
    pub redact_regex: Vec<String>,

    /// Trim trailing exit or logout command from output animation
    #[arg(long, default_value_t = true)]
    pub trim_exit: bool,

    /// Enable pause on mouse hover in SVG
    #[arg(long, default_value_t = true)]
    pub hover_pause: bool,

    /// Add drop shadow to SVG window frame
    #[arg(long, default_value_t = false)]
    pub shadow: bool,
}

#[derive(Args, Debug, Clone)]
pub struct ConvertArgs {
    /// Input Asciinema v2 (.cast) file path
    pub input: PathBuf,

    /// Output file destination path
    #[arg(short = 'o', long = "output")]
    pub output: PathBuf,

    /// Color theme name
    #[arg(long, default_value = "catppuccin-mocha")]
    pub theme: String,

    /// Window frame style for SVG outputs
    #[arg(long, value_enum, default_value_t = WindowStyle::Macos)]
    pub window_style: WindowStyle,

    /// Maximum idle duration between frames in seconds
    #[arg(long, default_value_t = 1.5)]
    pub max_wait: f64,

    /// Output format override (svg, html)
    #[arg(long, value_enum)]
    pub format: Option<OutputFormat>,

    /// Playback speed multiplier (e.g. 1.5 for 1.5x speed)
    #[arg(long, default_value_t = 1.0)]
    pub speed: f64,

    /// Window title displayed in the frame header
    #[arg(long, default_value = "termloom")]
    pub title: String,

    /// CSS font family fallback sequence
    #[arg(
        long,
        default_value = "'JetBrains Mono', 'Fira Code', 'Cascadia Code', 'SF Mono', Menlo, Consolas, monospace"
    )]
    pub font_family: String,

    /// Custom font URL to import in SVG
    #[arg(long)]
    pub font_url: Option<String>,

    /// Font size in pixels
    #[arg(long, default_value_t = 14)]
    pub font_size: u32,

    /// Line height multiplier
    #[arg(long, default_value_t = 1.35)]
    pub line_height: f64,

    /// Automatically mask secrets
    #[arg(long, default_value_t = false)]
    pub mask_secrets: bool,

    /// Custom regex patterns for secret redaction
    #[arg(long = "redact-regex")]
    pub redact_regex: Vec<String>,

    /// Enable pause on mouse hover in SVG
    #[arg(long, default_value_t = true)]
    pub hover_pause: bool,

    /// Add drop shadow to SVG window frame
    #[arg(long, default_value_t = false)]
    pub shadow: bool,
}

#[derive(Args, Debug, Clone)]
pub struct RunArgs {
    /// Path to scripted tape file (.tape)
    pub tape: PathBuf,

    /// Override output destination path defined in tape file
    #[arg(short = 'o', long = "output")]
    pub output: Option<PathBuf>,

    /// Automatically mask secrets
    #[arg(long, default_value_t = false)]
    pub mask_secrets: bool,

    /// Custom regex patterns for secret redaction
    #[arg(long = "redact-regex")]
    pub redact_regex: Vec<String>,
}

#[derive(Args, Debug, Clone)]
pub struct SnapshotArgs {
    /// Output file destination path (.svg)
    #[arg(default_value = "snapshot.svg")]
    pub output: PathBuf,

    /// Specific command to execute for the snapshot
    #[arg(short = 'c', long = "command")]
    pub command: Option<String>,

    /// Terminal column width
    #[arg(long, default_value_t = 100)]
    pub cols: u16,

    /// Terminal row count
    #[arg(long, default_value_t = 28)]
    pub rows: u16,

    /// Color theme name
    #[arg(long, default_value = "catppuccin-mocha")]
    pub theme: String,

    /// Window frame style
    #[arg(long, value_enum, default_value_t = WindowStyle::Macos)]
    pub window_style: WindowStyle,

    /// Window title displayed in the frame header
    #[arg(long, default_value = "termloom")]
    pub title: String,

    /// CSS font family fallback sequence
    #[arg(
        long,
        default_value = "'JetBrains Mono', 'Fira Code', 'Cascadia Code', 'SF Mono', Menlo, Consolas, monospace"
    )]
    pub font_family: String,

    /// Custom font URL to import in SVG
    #[arg(long)]
    pub font_url: Option<String>,

    /// Font size in pixels
    #[arg(long, default_value_t = 14)]
    pub font_size: u32,

    /// Line height multiplier
    #[arg(long, default_value_t = 1.35)]
    pub line_height: f64,

    /// Add drop shadow to window frame
    #[arg(long, default_value_t = true)]
    pub shadow: bool,
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Svg,
    Html,
    Cast,
}

impl OutputFormat {
    /// Infer output format from a file path extension.
    pub fn infer_from_path<P: AsRef<Path>>(path: P) -> Self {
        if let Some(ext) = path.as_ref().extension().and_then(|s| s.to_str()) {
            match ext.to_ascii_lowercase().as_str() {
                "html" | "htm" => OutputFormat::Html,
                "cast" => OutputFormat::Cast,
                _ => OutputFormat::Svg,
            }
        } else {
            OutputFormat::Svg
        }
    }
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowStyle {
    Macos,
    Squircle,
    None,
}
