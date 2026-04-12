use std::env;
use std::io::IsTerminal;

use crate::model::{ColorWhen, FileKind};

pub const RESET: &str = "\x1b[0m";
pub const DIM: &str = "\x1b[2m";

pub fn use_color(when: ColorWhen) -> bool {
    match when {
        ColorWhen::Always => true,
        ColorWhen::Never => false,
        ColorWhen::Auto => {
            if env::var_os("NO_COLOR").is_some() {
                return false;
            }
            std::io::stdout().is_terminal()
        }
    }
}

pub fn color_for_kind(kind: FileKind) -> &'static str {
    match kind {
        FileKind::Directory => "\x1b[34m",
        FileKind::Symlink => "\x1b[35m",
        FileKind::BrokenSymlink => "\x1b[31m",
        FileKind::Executable => "\x1b[32m",
        FileKind::Pipe => "\x1b[33m",
        FileKind::Socket => "\x1b[92m",
        FileKind::BlockDevice => "\x1b[31m",
        FileKind::CharDevice => "\x1b[93m",
        FileKind::Other => "\x1b[95m",
        FileKind::Regular => "",
    }
}
