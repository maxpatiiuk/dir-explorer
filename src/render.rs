use std::path::Path;
use std::{env, fs};

use crate::color::{DIM, RESET};
use crate::format_meta::{
    format_mode_octal, format_size, format_time_long_iso, resolve_group, resolve_user,
};
use crate::format_name::colorized_name;
use crate::fs::{collect_entries, collect_entry_for_path, is_directory_for_recursion};
use crate::model::{Entry, FileKind, Options, ViewMode};
use crate::sort::sort_entries;

pub fn render_paths(options: &Options, use_color: bool) -> Result<Vec<String>, String> {
    let mut out = Vec::new();
    let mut file_operands: Vec<Entry> = Vec::new();
    let mut directory_operands = Vec::new();

    for path in &options.paths {
        let operand = collect_entry_for_path(path, Some(path.to_string_lossy().into_owned()))?;
        if operand.kind == FileKind::Directory {
            directory_operands.push(path.clone());
        } else {
            file_operands.push(operand);
        }
    }

    if !file_operands.is_empty() {
        sort_entries(&mut file_operands, options.sort_mode, options.reverse);
        for entry in file_operands {
            out.push(render_entry(&entry, options, use_color));
        }
    }

    let show_dir_headers = directory_operands.len() > 1 || !out.is_empty();
    for path in &directory_operands {
        if !out.is_empty() {
            out.push(String::new());
        }

        if show_dir_headers {
            out.push(format!("{}:", path.display()));
        }

        if options.recursive {
            render_tree(path, "", true, options, use_color, &mut out)?;
        } else {
            let mut entries = collect_entries(path, options)?;
            let hidden_count = apply_home_directory_overrides(path, options, &mut entries);
            sort_entries(&mut entries, options.sort_mode, options.reverse);

            if should_show_home_hidden_count(path, options) {
                out.push(format!("({hidden_count} hidden)"));
            }

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
            let child_prefix = if last {
                format!("{prefix}    ")
            } else {
                format!("{prefix}│   ")
            };
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
            let size = if entry.kind == FileKind::Directory {
                String::new()
            } else {
                format_size(entry.size, options.human_readable)
            };
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
            let size = if entry.kind == FileKind::Directory {
                String::new()
            } else {
                format_size(entry.size, options.human_readable)
            };
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

fn should_show_home_hidden_count(path: &Path, options: &Options) -> bool {
    options.view_mode != ViewMode::Long && is_home_directory(path)
}

fn apply_home_directory_overrides(
    path: &Path,
    options: &Options,
    entries: &mut Vec<Entry>,
) -> usize {
    if !should_show_home_hidden_count(path, options) {
        return 0;
    }

    let mut filtered = Vec::with_capacity(entries.len());
    let mut hidden_count = 0usize;

    for mut entry in entries.drain(..) {
        // Some home dir files can't be moved.
        // Some apps don't respect XDG dirs and clutter the home dir
        // Hide the directories I almost never need to open to reduce noise
        match entry.name.as_str() {
            ".CFUserTextEncoding"
            | ".Trash"
            | ".android"
            | ".bash_history"
            | ".boto"
            | ".cargo"
            | ".claude"
            | ".claude.json"
            | ".condarc"
            | ".copilot"
            | ".docker"
            | ".gemini"
            | ".gitconfig"
            | ".gnupg"
            | ".gsutil"
            | ".lesshst"
            | ".node_repl_history"
            | ".npm"
            | ".npmrc"
            | ".redhat"
            | ".rustup"
            | ".screenrc"
            | ".ssh"
            | ".storybook"
            | ".vim"
            | ".viminfo"
            | ".vimrc"
            | ".vscode"
            | ".yarn"
            | ".zsh_history"
            | ".zsh_sessions"
            | ".zshenv"
            | "Desktop"
            | "Downloads"
            | "Movies"
            | "Music"
            | "Pictures"
            | "Public" => {
                hidden_count += 1;
                continue;
            }
            ".DS_Store" => {
                continue;
            }
            "d" if entry.kind == FileKind::Symlink && entry.groups_with_directories => {
                entry.kind = FileKind::Directory;
                entry.symlink_target = None;
            }
            _ => {}
        }
        filtered.push(entry);
    }

    *entries = filtered;
    hidden_count
}

fn is_home_directory(path: &Path) -> bool {
    let Some(home_os) = env::var_os("HOME") else {
        return false;
    };
    let home_path = std::path::PathBuf::from(home_os);
    let resolved_input = fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    let resolved_home = fs::canonicalize(&home_path).unwrap_or(home_path);
    resolved_input == resolved_home
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::model::{Entry, FileKind, Options, SortMode, ViewMode};
    use crate::render::{apply_home_directory_overrides, render_paths};

    fn make_temp_dir() -> PathBuf {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("dir_explorer_render_test_{seed}"));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn mixed_operands_show_files_then_directory_sections() {
        let root = make_temp_dir();
        let file_path = root.join("single.txt");
        let dir_path = root.join("folder");
        fs::write(&file_path, b"x").unwrap();
        fs::create_dir_all(&dir_path).unwrap();
        fs::write(dir_path.join("inside.txt"), b"y").unwrap();

        let mut options = Options::default();
        options.view_mode = ViewMode::Zero;
        options.sort_mode = SortMode::Name;
        options.paths = vec![file_path.clone(), dir_path.clone()];

        let lines = render_paths(&options, false).unwrap();
        assert_eq!(lines[0], file_path.to_string_lossy());
        assert_eq!(lines[1], "");
        assert_eq!(lines[2], format!("{}:", dir_path.display()));
        assert_eq!(lines[3], "inside.txt");

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn multiple_directories_have_single_blank_between_sections() {
        let root = make_temp_dir();
        let dir_a = root.join("a");
        let dir_b = root.join("b");
        fs::create_dir_all(&dir_a).unwrap();
        fs::create_dir_all(&dir_b).unwrap();
        fs::write(dir_a.join("x"), b"x").unwrap();
        fs::write(dir_b.join("y"), b"y").unwrap();

        let mut options = Options::default();
        options.view_mode = ViewMode::Zero;
        options.sort_mode = SortMode::Name;
        options.paths = vec![dir_a.clone(), dir_b.clone()];

        let lines = render_paths(&options, false).unwrap();
        assert!(lines
            .iter()
            .any(|line| line == &format!("{}:", dir_a.display())));
        assert!(lines
            .iter()
            .any(|line| line == &format!("{}:", dir_b.display())));

        let blank_count = lines.iter().filter(|line| line.is_empty()).count();
        assert_eq!(blank_count, 1);

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn directory_size_field_is_blank_in_default_view() {
        let root = make_temp_dir();
        let dir_path = root.join("folder");
        fs::create_dir_all(&dir_path).unwrap();

        let mut options = Options::default();
        options.view_mode = ViewMode::Default;
        options.sort_mode = SortMode::Name;
        options.paths = vec![root.clone()];

        let lines = render_paths(&options, false).unwrap();
        let line = lines
            .iter()
            .find(|line| line.ends_with(" folder"))
            .expect("folder line should exist");

        assert!(line.starts_with("         "));

        fs::remove_dir_all(root).unwrap();
    }

    fn entry(name: &str, kind: FileKind) -> Entry {
        Entry {
            path: PathBuf::from(name),
            name: name.to_string(),
            kind,
            groups_with_directories: kind == FileKind::Directory,
            mode: 0,
            nlink: 1,
            uid: 0,
            gid: 0,
            size: 0,
            modified: UNIX_EPOCH,
            symlink_target: None,
        }
    }

    #[test]
    fn home_overrides_hide_expected_noise_and_count() {
        let mut entries = vec![
            entry(".CFUserTextEncoding", FileKind::Regular),
            entry("Music", FileKind::Directory),
            entry("Desktop", FileKind::Directory),
            entry(".DS_Store", FileKind::Regular),
            entry("keep", FileKind::Regular),
        ];

        let mut options = Options::default();
        options.view_mode = ViewMode::Default;

        let count = apply_home_directory_overrides(Path::new("/tmp"), &options, &mut entries);
        assert_eq!(count, 0);

        let count = apply_home_directory_overrides(
            Path::new(&std::env::var("HOME").unwrap()),
            &options,
            &mut entries,
        );
        assert_eq!(count, 3);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].name, "keep");
    }

    #[test]
    fn home_overrides_render_d_symlink_as_directory() {
        let mut d = entry("d", FileKind::Symlink);
        d.groups_with_directories = true;
        d.symlink_target = Some(PathBuf::from("/tmp/somewhere"));
        let mut entries = vec![d];

        let mut options = Options::default();
        options.view_mode = ViewMode::Default;

        let _ = apply_home_directory_overrides(
            Path::new(&std::env::var("HOME").unwrap()),
            &options,
            &mut entries,
        );

        assert_eq!(entries[0].kind, FileKind::Directory);
        assert!(entries[0].symlink_target.is_none());
    }
}
