use std::path::Path;

use crate::color::{DIM, RESET};
use crate::format_meta::{
    format_mode_octal, format_size, format_time_long_iso, resolve_group, resolve_user,
};
use crate::format_name::colorized_name;
use crate::fs::{collect_entries, collect_entry_for_path, is_directory_for_recursion};
use crate::home_overrides::apply_home_directory_overrides;
use crate::model::{Entry, FileKind, Options, ViewMode};
use crate::sort::sort_entries;
use crate::theme::is_black_hole_dir_name;

#[derive(Clone, Copy)]
struct LongViewWidths {
    user: usize,
    group: usize,
    size: usize,
}

fn is_within_black_hole_dir(path: &Path) -> bool {
    path.components().any(|component| {
        component
            .as_os_str()
            .to_str()
            .is_some_and(is_black_hole_dir_name)
    })
}

fn shows_size(entry: &Entry) -> bool {
    entry.kind != FileKind::Directory
        && !(entry.kind == FileKind::Symlink && entry.groups_with_directories)
}

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
        let widths = compute_long_view_widths(&file_operands, options);
        for entry in file_operands {
            out.push(render_entry(&entry, options, use_color, widths));
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
            render_tree(path, "", 0, options, use_color, &mut out)?;
        } else {
            let mut entries = collect_entries(path, options)?;
            let hidden_count = apply_home_directory_overrides(path, options, &mut entries);
            sort_entries(&mut entries, options.sort_mode, options.reverse);
            let widths = compute_long_view_widths(&entries, options);

            if options.view_mode != ViewMode::Long && hidden_count > 0 {
                out.push(format!("({hidden_count} hidden)"));
            }

            for entry in entries {
                out.push(render_entry(&entry, options, use_color, widths));
            }
        }
    }

    Ok(out)
}

fn render_tree(
    path: &Path,
    prefix: &str,
    depth: usize,
    options: &Options,
    use_color: bool,
    out: &mut Vec<String>,
) -> Result<(), String> {
    let mut entries = collect_entries(path, options)?;
    sort_entries(&mut entries, options.sort_mode, options.reverse);
    let widths = compute_long_view_widths(&entries, options);

    for (index, entry) in entries.iter().enumerate() {
        let last = index + 1 == entries.len();
        let branch = if last { "└── " } else { "├── " };
        let (meta, name) = render_entry_parts(entry, options, use_color, widths);
        let tree = if depth == 0 {
            String::new()
        } else {
            format!("{prefix}{branch}")
        };
        let line = match (meta.is_empty(), tree.is_empty()) {
            (true, true) => name,
            (true, false) => format!("{tree}{name}"),
            (false, true) => format!("{meta} {name}"),
            (false, false) => format!("{meta} {tree}{name}"),
        };
        out.push(line);

        let should_recurse = is_directory_for_recursion(entry)
            && (is_within_black_hole_dir(path) || !is_black_hole_dir_name(&entry.name));

        if should_recurse {
            let child_prefix = if last {
                format!("{prefix}    ")
            } else {
                format!("{prefix}│   ")
            };
            render_tree(
                &entry.path,
                &child_prefix,
                depth + 1,
                options,
                use_color,
                out,
            )?;
        }
    }

    Ok(())
}

fn render_entry(
    entry: &Entry,
    options: &Options,
    use_color: bool,
    widths: LongViewWidths,
) -> String {
    let (meta, name) = render_entry_parts(entry, options, use_color, widths);
    if meta.is_empty() {
        name
    } else {
        format!("{meta} {name}")
    }
}

fn render_entry_parts(
    entry: &Entry,
    options: &Options,
    use_color: bool,
    widths: LongViewWidths,
) -> (String, String) {
    let name = colorized_name(entry, use_color);

    match options.view_mode {
        ViewMode::Zero => (String::new(), name),
        ViewMode::Default => {
            let size = if !shows_size(entry) {
                String::new()
            } else {
                format_size(entry.size, options.human_readable)
            };
            let time = format_time_long_iso(entry.modified);
            if use_color {
                (format!("{DIM}{size:>8} {time}{RESET}"), name)
            } else {
                (format!("{size:>8} {time}"), name)
            }
        }
        ViewMode::Long => {
            let mode = format_mode_octal(entry.mode);
            let user = resolve_user(entry.uid);
            let group = resolve_group(entry.gid);
            let size = if !shows_size(entry) {
                String::new()
            } else {
                format_size(entry.size, options.human_readable)
            };
            let time = format_time_long_iso(entry.modified);
            if use_color {
                (
                    format!(
                        "{DIM}{mode:>4} {user:<user_width$} {group:<group_width$} {size:>size_width$} {time}{RESET}",
                        user_width = widths.user,
                        group_width = widths.group,
                        size_width = widths.size,
                    ),
                    name,
                )
            } else {
                (
                    format!(
                        "{mode:>4} {user:<user_width$} {group:<group_width$} {size:>size_width$} {time}",
                        user_width = widths.user,
                        group_width = widths.group,
                        size_width = widths.size,
                    ),
                    name,
                )
            }
        }
    }
}

fn compute_long_view_widths(entries: &[Entry], options: &Options) -> LongViewWidths {
    if options.view_mode != ViewMode::Long {
        return LongViewWidths {
            user: 1,
            group: 1,
            size: 0,
        };
    }

    let mut widths = LongViewWidths {
        user: 1,
        group: 1,
        size: 0,
    };

    for entry in entries {
        widths.user = widths.user.max(resolve_user(entry.uid).len());
        widths.group = widths.group.max(resolve_group(entry.gid).len());

        let size = if !shows_size(entry) {
            String::new()
        } else {
            format_size(entry.size, options.human_readable)
        };
        widths.size = widths.size.max(size.len());
    }

    widths
}

#[cfg(test)]
mod tests {
    use super::{compute_long_view_widths, render_entry};
    use std::fs;
    use std::os::unix::fs::symlink;
    use std::path::{Path, PathBuf};
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::home_overrides::apply_home_directory_overrides;
    use crate::model::{Entry, FileKind, Options, SortMode, ViewMode};
    use crate::render::render_paths;

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

    #[test]
    fn directory_symlink_size_field_is_blank_in_default_view() {
        let root = make_temp_dir();
        let target_dir = root.join("target_dir");
        let link_path = root.join("etc");
        fs::create_dir_all(&target_dir).unwrap();
        symlink(&target_dir, &link_path).unwrap();

        let mut options = Options::default();
        options.view_mode = ViewMode::Default;
        options.sort_mode = SortMode::Name;
        options.paths = vec![root.clone()];

        let lines = render_paths(&options, false).unwrap();
        let line = lines
            .iter()
            .find(|line| line.contains(" etc -> "))
            .expect("directory symlink line should exist");

        assert!(line.starts_with("         "));

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn recursive_skips_black_hole_dirs_by_default() {
        let root = make_temp_dir();
        let git_dir = root.join(".git");
        let src_dir = root.join("src");
        fs::create_dir_all(&git_dir).unwrap();
        fs::create_dir_all(&src_dir).unwrap();
        fs::write(git_dir.join("config"), b"x").unwrap();
        fs::write(src_dir.join("main.rs"), b"x").unwrap();

        let mut options = Options::default();
        options.view_mode = ViewMode::Zero;
        options.sort_mode = SortMode::Name;
        options.recursive = true;
        options.paths = vec![root.clone()];

        let lines = render_paths(&options, false).unwrap();
        assert!(lines.iter().any(|line| line.contains(".git")));
        assert!(!lines.iter().any(|line| line.contains("config")));
        assert!(lines.iter().any(|line| line.contains("src")));
        assert!(lines.iter().any(|line| line.contains("main.rs")));

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn recursive_descends_when_starting_inside_black_hole_dir() {
        let root = make_temp_dir();
        let git_dir = root.join(".git");
        fs::create_dir_all(&git_dir).unwrap();
        fs::write(git_dir.join("config"), b"x").unwrap();

        let mut options = Options::default();
        options.view_mode = ViewMode::Zero;
        options.sort_mode = SortMode::Name;
        options.recursive = true;
        options.paths = vec![git_dir.clone()];

        let lines = render_paths(&options, false).unwrap();
        assert!(lines.iter().any(|line| line.contains("config")));

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn directory_symlink_size_is_hidden_in_long_view() {
        let mut entry = entry("etc", FileKind::Symlink);
        entry.groups_with_directories = true;
        entry.size = 11;
        entry.symlink_target = Some(PathBuf::from("private/etc"));

        let mut options = Options::default();
        options.view_mode = ViewMode::Long;

        let widths = compute_long_view_widths(&[entry.clone()], &options);
        let line = render_entry(&entry, &options, false, widths);

        assert!(!line.contains("11"));
        assert!(line.contains(" etc -> private/etc"));
    }

    fn entry(name: &str, kind: FileKind) -> Entry {
        Entry {
            path: PathBuf::from(name),
            name: name.to_string(),
            kind,
            groups_with_directories: kind == FileKind::Directory,
            color_override: None,
            mode: 0,
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

    #[test]
    fn home_overrides_gray_selected_home_directories() {
        let mut entries = vec![
            entry(".config", FileKind::Directory),
            entry(".local", FileKind::Directory),
            entry("Applications", FileKind::Directory),
            entry("Documents", FileKind::Directory),
            entry("Library", FileKind::Directory),
            entry("src", FileKind::Directory),
        ];

        let mut options = Options::default();
        options.view_mode = ViewMode::Default;

        let _ = apply_home_directory_overrides(
            Path::new(&std::env::var("HOME").unwrap()),
            &options,
            &mut entries,
        );

        let should_be_gray = [".config", ".local", "Applications", "Documents", "Library"];
        for name in should_be_gray {
            let item = entries.iter().find(|entry| entry.name == name).unwrap();
            assert_eq!(item.color_override.as_deref(), Some("gray"));
        }

        let src = entries.iter().find(|entry| entry.name == "src").unwrap();
        assert_eq!(src.color_override.as_deref(), Some("gray"));
    }

    #[test]
    fn home_overrides_keep_include_list_ungrayed() {
        let mut entries = vec![
            entry("d", FileKind::Directory),
            entry("g", FileKind::Directory),
            entry("j", FileKind::Directory),
            entry("e", FileKind::Directory),
            entry("random", FileKind::Directory),
        ];

        let mut options = Options::default();
        options.view_mode = ViewMode::Default;

        let _ = apply_home_directory_overrides(
            Path::new(&std::env::var("HOME").unwrap()),
            &options,
            &mut entries,
        );

        for name in ["d", "g", "j", "e"] {
            let item = entries.iter().find(|entry| entry.name == name).unwrap();
            assert!(item.color_override.is_none());
        }

        let random = entries.iter().find(|entry| entry.name == "random").unwrap();
        assert_eq!(random.color_override.as_deref(), Some("gray"));
    }
}
