use std::path::Path;

use crate::color::{DIM, RESET};
use crate::format_meta::{format_mode_octal, format_size, format_time_long_iso, resolve_group, resolve_user};
use crate::format_name::colorized_name;
use crate::fs::{collect_entries, is_directory_for_recursion};
use crate::model::{Entry, Options, ViewMode};
use crate::sort::sort_entries;

pub fn render_paths(options: &Options, use_color: bool) -> Result<Vec<String>, String> {
    let mut out = Vec::new();
    let show_headers = options.paths.len() > 1;

    for (index, path) in options.paths.iter().enumerate() {
        if show_headers {
            if index > 0 {
                out.push(String::new());
            }
            out.push(format!("{}:", path.display()));
        }

        if options.recursive {
            render_tree(path, "", true, options, use_color, &mut out)?;
        } else {
            let mut entries = collect_entries(path, options)?;
            sort_entries(&mut entries, options.sort_mode, options.reverse);
            for entry in entries {
                out.push(render_entry(&entry, options, use_color));
            }
        }
    }

    Ok(out)
}

fn render_tree(
    path: &Path,
    prefix: &str,
    is_root: bool,
    options: &Options,
    use_color: bool,
    out: &mut Vec<String>,
) -> Result<(), String> {
    if is_root {
        out.push(format!("{}/", path.display()));
    }

    let mut entries = collect_entries(path, options)?;
    sort_entries(&mut entries, options.sort_mode, options.reverse);

    for (index, entry) in entries.iter().enumerate() {
        let last = index + 1 == entries.len();
        let branch = if last { "└── " } else { "├── " };
        let line = render_entry(entry, options, use_color);
        out.push(format!("{prefix}{branch}{line}"));

        if is_directory_for_recursion(entry) {
            let child_prefix = if last { format!("{prefix}    ") } else { format!("{prefix}│   ") };
            render_tree(&entry.path, &child_prefix, false, options, use_color, out)?;
        }
    }

    Ok(())
}

fn render_entry(entry: &Entry, options: &Options, use_color: bool) -> String {
    let name = colorized_name(entry, use_color);

    match options.view_mode {
        ViewMode::Zero => name,
        ViewMode::Default => {
            let size = format_size(entry.size, options.human_readable);
            let time = format_time_long_iso(entry.modified);
            if use_color {
                format!("{DIM}{size:>8} {time}{RESET} {name}")
            } else {
                format!("{size:>8} {time} {name}")
            }
        }
        ViewMode::Long => {
            let mode = format_mode_octal(entry.mode);
            let links = entry.nlink;
            let user = resolve_user(entry.uid);
            let group = resolve_group(entry.gid);
            let size = format_size(entry.size, options.human_readable);
            let time = format_time_long_iso(entry.modified);
            if use_color {
                format!(
                    "{DIM}{mode:>4} {links:>2} {user:<8} {group:<8} {size:>8} {time}{RESET} {name}"
                )
            } else {
                format!("{mode:>4} {links:>2} {user:<8} {group:<8} {size:>8} {time} {name}")
            }
        }
    }
}
