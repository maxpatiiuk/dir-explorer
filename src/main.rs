mod cli;
mod color;
mod format_meta;
mod format_name;
mod fs;
mod model;
mod render;
mod sort;
mod theme;

use std::process::ExitCode;
use std::{io, io::Write};

use cli::{help_text, parse_args};
use color::use_color;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let options = match parse_args(args) {
        Ok(options) => options,
        Err(message) => {
            if message == help_text() {
                println!("{message}");
                return ExitCode::SUCCESS;
            }
            eprintln!("{message}");
            eprintln!();
            eprintln!("{}", help_text());
            return ExitCode::from(2);
        }
    };

    let use_colors = use_color(options.color_when);

    match render::render_paths(&options, use_colors) {
        Ok(lines) => {
            let stdout = io::stdout();
            let mut handle = io::BufWriter::new(stdout.lock());
            for line in lines {
                if let Err(error) = writeln!(handle, "{line}") {
                    if error.kind() == io::ErrorKind::BrokenPipe {
                        return ExitCode::SUCCESS;
                    }
                    eprintln!("{error}");
                    return ExitCode::from(1);
                }
            }
            let _ = handle.flush();
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(1)
        }
    }
}
