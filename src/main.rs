//! TermLoom: Modern Cross-Platform Terminal Session Recorder & Vector Animator.

pub mod cli;
pub mod emulator;
pub mod exporters;
pub mod pty;
pub mod recorder;
pub mod tape;
pub mod themes;

use clap::Parser;
use cli::{Cli, Commands, ConvertArgs, OutputFormat, RecordArgs, RunArgs, SnapshotArgs};
use exporters::{HtmlRenderer, SvgRenderer};
use recorder::{read_cast, write_cast, Compressor, Redactor};
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;
use tape::{parse_tape, run_tape};
use themes::find_theme;

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Record(args) => handle_record(args),
        Commands::Convert(args) => handle_convert(args),
        Commands::Run(args) => handle_run(args),
        Commands::Snapshot(args) => handle_snapshot(args),
    };

    if let Err(err) = result {
        eprintln!("[termloom] Error: {}", err);
        std::process::exit(1);
    }
}

fn create_redactor(
    mask_secrets: bool,
    custom_regexes: &[String],
) -> Result<Option<Redactor>, Box<dyn std::error::Error + Send + Sync>> {
    if !mask_secrets && custom_regexes.is_empty() {
        return Ok(None);
    }
    let redactor = if custom_regexes.is_empty() {
        Redactor::standard()
    } else {
        Redactor::with_custom(custom_regexes)?
    };
    Ok(Some(redactor))
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
        print_summary(
            &args.output.display().to_string(),
            file_size,
            raw_duration,
            raw_duration,
            events.len(),
            events.len(),
            0,
        );
        return Ok(());
    }

    // Configure compressor with redaction and trim-exit
    let redactor = create_redactor(args.mask_secrets, &args.redact_regex)?;
    let compressor = Compressor::new(args.cols as usize, args.rows as usize, args.max_wait)
        .with_redactor(redactor)
        .with_trim_exit(args.trim_exit);

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
            )
            .with_font_url(args.font_url.as_deref())
            .with_hover_pause(args.hover_pause)
            .with_shadow(args.shadow);
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

    print_summary(
        &args.output.display().to_string(),
        file_size,
        raw_duration,
        compressed_duration,
        events.len(),
        frames.len(),
        0,
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
    let raw_duration = events.last().map(|e| e.timestamp).unwrap_or(0.0);

    let redactor = create_redactor(args.mask_secrets, &args.redact_regex)?;
    let compressor = Compressor::new(cols, rows, args.max_wait).with_redactor(redactor);
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
    let compressed_duration: f64 = frames.iter().map(|f| f.duration).sum();

    let rendered = match format {
        OutputFormat::Svg => {
            let renderer = SvgRenderer::new(
                palette,
                args.window_style,
                &args.title,
                &args.font_family,
                args.font_size,
                args.line_height,
            )
            .with_font_url(args.font_url.as_deref())
            .with_hover_pause(args.hover_pause)
            .with_shadow(args.shadow);
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

    print_summary(
        &args.output.display().to_string(),
        file_size,
        raw_duration,
        compressed_duration,
        events.len(),
        frames.len(),
        0,
    );

    Ok(())
}

fn handle_run(args: RunArgs) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let content = std::fs::read_to_string(&args.tape)?;
    let script = parse_tape(&content)?;

    let target_output = args
        .output
        .or(script.output.clone())
        .unwrap_or_else(|| Path::new("tape_output.svg").to_path_buf());

    println!(
        "[termloom] Running tape script '{}' -> {}",
        args.tape.display(),
        target_output.display()
    );

    let events = run_tape(&script)?;
    let raw_duration = events.last().map(|e| e.timestamp).unwrap_or(0.0);

    let palette = find_theme(&script.theme).unwrap_or(&themes::CATPPUCCIN_MOCHA);
    let redactor = create_redactor(args.mask_secrets, &args.redact_regex)?;
    let compressor = Compressor::new(script.cols as usize, script.rows as usize, script.max_wait)
        .with_redactor(redactor);
    let frames = compressor.process(&events);
    let compressed_duration: f64 = frames.iter().map(|f| f.duration).sum();

    let format = OutputFormat::infer_from_path(&target_output);
    let rendered = match format {
        OutputFormat::Svg => {
            let renderer = SvgRenderer::new(
                palette,
                script.window_style,
                &script.title,
                "'JetBrains Mono', 'Fira Code', monospace",
                script.font_size,
                1.35,
            );
            renderer.render(&frames, script.cols as usize, script.rows as usize)
        }
        OutputFormat::Html => {
            let renderer = HtmlRenderer::new(palette, &script.title);
            renderer.render(&frames, script.cols as usize, script.rows as usize)
        }
        OutputFormat::Cast => {
            let mut file = File::create(&target_output)?;
            write_cast(
                &mut file,
                &events,
                script.cols as usize,
                script.rows as usize,
                &script.title,
            )?;
            let file_size = file.metadata().map(|m| m.len()).unwrap_or(0);
            print_summary(
                &target_output.display().to_string(),
                file_size,
                raw_duration,
                raw_duration,
                events.len(),
                events.len(),
                0,
            );
            return Ok(());
        }
    };

    let mut out_file = File::create(&target_output)?;
    out_file.write_all(rendered.as_bytes())?;
    let file_size = out_file.metadata().map(|m| m.len()).unwrap_or(0);

    print_summary(
        &target_output.display().to_string(),
        file_size,
        raw_duration,
        compressed_duration,
        events.len(),
        frames.len(),
        0,
    );

    Ok(())
}

fn handle_snapshot(args: SnapshotArgs) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let palette = find_theme(&args.theme).unwrap_or(&themes::CATPPUCCIN_MOCHA);

    println!(
        "[termloom] Generating terminal snapshot ({}x{}, theme: {})",
        args.cols, args.rows, palette.name
    );

    let events = pty::run_session(args.command.as_deref(), args.cols, args.rows)?;
    let compressor = Compressor::new(args.cols as usize, args.rows as usize, 1.0);
    let frames = compressor.process(&events);

    // Take the final captured frame for the static snapshot
    let snapshot_frame = frames.last().cloned().unwrap_or_else(|| {
        let grid = crate::emulator::grid::ScreenGrid::new(args.cols as usize, args.rows as usize);
        recorder::Frame::new(grid, 1.0, 0.0)
    });

    let renderer = SvgRenderer::new(
        palette,
        args.window_style,
        &args.title,
        &args.font_family,
        args.font_size,
        args.line_height,
    )
    .with_font_url(args.font_url.as_deref())
    .with_shadow(args.shadow);

    let rendered = renderer.render(&[snapshot_frame], args.cols as usize, args.rows as usize);

    let mut file = File::create(&args.output)?;
    file.write_all(rendered.as_bytes())?;
    let file_size = file.metadata().map(|m| m.len()).unwrap_or(0);

    println!(
        "[termloom] Snapshot saved to {} ({} bytes)",
        args.output.display(),
        file_size
    );

    Ok(())
}

fn print_summary(
    path: &str,
    bytes: u64,
    raw_dur: f64,
    comp_dur: f64,
    raw_frames: usize,
    dedup_frames: usize,
    _secrets: usize,
) {
    let reduction = if raw_frames > 0 {
        ((1.0 - (dedup_frames as f64 / raw_frames as f64)) * 100.0).max(0.0)
    } else {
        0.0
    };

    println!("\n[termloom] Session Summary");
    println!("  Output:      {}", path);
    println!("  Size:        {:.1} KB", bytes as f64 / 1024.0);
    println!(
        "  Duration:    {:.1}s raw -> {:.1}s animated",
        raw_dur, comp_dur
    );
    println!(
        "  Frames:      {} raw -> {} deduplicated ({:.1}% reduction)",
        raw_frames, dedup_frames, reduction
    );
}
