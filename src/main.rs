//! TermLoom: Modern Cross-Platform Terminal Session Recorder & Vector Animator.

pub mod cli;
pub mod emulator;
pub mod exporters;
pub mod pty;
pub mod recorder;
pub mod themes;

use clap::Parser;
use cli::{Cli, Commands, ConvertArgs, OutputFormat, RecordArgs};
use exporters::{HtmlRenderer, SvgRenderer};
use recorder::{read_cast, write_cast, Compressor};
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;
use themes::find_theme;

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Record(args) => handle_record(args),
        Commands::Convert(args) => handle_convert(args),
    };

    if let Err(err) = result {
        eprintln!("[termloom] Error: {}", err);
        std::process::exit(1);
    }
}

fn handle_record(args: RecordArgs) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let palette = find_theme(&args.theme).unwrap_or(&themes::CATPPUCCIN_MOCHA);
    let format = args
        .format
        .unwrap_or_else(|| OutputFormat::infer_from_path(&args.output));

    if args.command.is_none() {
        println!(
            "[termloom] Recording session ({}x{}, theme: {}). Press Ctrl+D or type exit to finish.",
            args.cols, args.rows, palette.name
        );
    }

    // Run PTY session
    let events = pty::run_session(args.command.as_deref(), args.cols, args.rows)?;
    let raw_duration = events.last().map(|e| e.timestamp).unwrap_or(0.0);

    // If recording directly to Cast format, write raw events
    if format == OutputFormat::Cast {
        let mut file = File::create(&args.output)?;
        write_cast(
            &mut file,
            &events,
            args.cols as usize,
            args.rows as usize,
            &args.title,
        )?;
        let file_size = file.metadata().map(|m| m.len()).unwrap_or(0);
        println!(
            "\n[termloom] Successfully wrote Asciinema v2 recording to {} ({:.1}s, {} bytes)",
            args.output.display(),
            raw_duration,
            file_size
        );
        return Ok(());
    }

    // Compress and deduplicate frames
    let compressor = Compressor::new(args.cols as usize, args.rows as usize, args.max_wait);
    let frames = compressor.process(&events);
    let compressed_duration: f64 = frames.iter().map(|f| f.duration).sum();

    // Export to selected format
    let rendered = match format {
        OutputFormat::Svg => {
            let renderer = SvgRenderer::new(
                palette,
                args.window_style,
                &args.title,
                &args.font_family,
                args.font_size,
                args.line_height,
            );
            renderer.render(&frames, args.cols as usize, args.rows as usize)
        }
        OutputFormat::Html => {
            let renderer = HtmlRenderer::new(palette, &args.title);
            renderer.render(&frames, args.cols as usize, args.rows as usize)
        }
        OutputFormat::Cast => unreachable!(),
    };

    let mut file = File::create(&args.output)?;
    file.write_all(rendered.as_bytes())?;
    let file_size = file.metadata().map(|m| m.len()).unwrap_or(0);

    println!(
        "\n[termloom] Exported {} frames to {} ({:.1}s raw -> {:.1}s animated, {} bytes)",
        frames.len(),
        args.output.display(),
        raw_duration,
        compressed_duration,
        file_size
    );

    Ok(())
}

fn handle_convert(args: ConvertArgs) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let palette = find_theme(&args.theme).unwrap_or(&themes::CATPPUCCIN_MOCHA);
    let format = args
        .format
        .unwrap_or_else(|| OutputFormat::infer_from_path(&args.output));

    if !Path::new(&args.input).exists() {
        return Err(format!("Input file not found: {}", args.input.display()).into());
    }

    let file = File::open(&args.input)?;
    let reader = BufReader::new(file);
    let (cols, rows, events) = read_cast(reader)?;

    let compressor = Compressor::new(cols, rows, args.max_wait);
    let mut frames = compressor.process(&events);

    // Apply speed multiplier if requested
    if (args.speed - 1.0).abs() > 0.001 && args.speed > 0.0 {
        let speed_factor = 1.0 / args.speed;
        let mut cur_time = 0.0;
        for frame in &mut frames {
            frame.duration *= speed_factor;
            frame.timestamp = cur_time;
            cur_time += frame.duration;
        }
    }

    let rendered = match format {
        OutputFormat::Svg => {
            let renderer = SvgRenderer::new(
                palette,
                args.window_style,
                &args.title,
                &args.font_family,
                args.font_size,
                args.line_height,
            );
            renderer.render(&frames, cols, rows)
        }
        OutputFormat::Html => {
            let renderer = HtmlRenderer::new(palette, &args.title);
            renderer.render(&frames, cols, rows)
        }
        OutputFormat::Cast => {
            return Err("Conversion target cannot be cast format".into());
        }
    };

    let mut out_file = File::create(&args.output)?;
    out_file.write_all(rendered.as_bytes())?;
    let file_size = out_file.metadata().map(|m| m.len()).unwrap_or(0);

    println!(
        "[termloom] Converted {} into {} ({} frames, {} bytes)",
        args.input.display(),
        args.output.display(),
        frames.len(),
        file_size
    );

    Ok(())
}
