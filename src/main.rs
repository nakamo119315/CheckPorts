//! ports - Display listening TCP ports and application information
//!
//! A CLI tool for macOS that shows which applications are using which ports,
//! helping developers manage their local development environment.

use clap::Parser;
use std::process::ExitCode;

mod app_detector;
mod error;
mod models;
mod output;
mod port_scanner;
mod process_info;

use error::PortsError;
use output::OutputFormat;

/// Display listening TCP ports and application information.
///
/// Shows which ports are in use, what applications are using them,
/// and provides information to help decide if a process can be stopped.
#[derive(Parser, Debug)]
#[command(name = "ports")]
#[command(version)]
#[command(about = "リッスン中のTCPポートとアプリケーション情報を表示")]
#[command(
    long_about = "リッスン中のTCPポートとアプリケーション情報を表示します。\n\n\
    開発者がローカル環境でどのポートがどのアプリケーションに\n\
    使用されているかを確認し、プロセスを停止してよいかを\n\
    判断するための情報を提供します。"
)]
struct Args {
    /// Output in JSON format for scripting and automation
    #[arg(short, long)]
    json: bool,
}

fn main() -> ExitCode {
    let args = Args::parse();

    match run(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {}", error);
            eprintln!("hint: {}", error.hint());
            ExitCode::from(1)
        }
    }
}

fn run(args: &Args) -> Result<(), PortsError> {
    // Scan for listening ports
    let mut entries = port_scanner::scan_listening_ports()?;

    // Sort by port number (ascending)
    entries.sort_by_key(|e| e.port);

    // Enrich with detailed process info
    for entry in &mut entries {
        if let Err(e) = process_info::enrich_process_info(&mut entry.process) {
            // Log warning but continue with partial info
            eprintln!("warning: Failed to get details for PID {}: {}", entry.process.pid, e);
        }
    }

    // Detect application types
    for entry in &mut entries {
        let app_type = app_detector::detect_app_type(&entry.process);
        entry.app_type = Some(app_type);
    }

    // Output results
    let format = if args.json {
        OutputFormat::Json
    } else {
        OutputFormat::Table
    };

    output::print_entries(&entries, format);

    Ok(())
}
