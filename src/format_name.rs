use std::env;
use std::path::{Path, PathBuf};

use crate::color::{color_for_kind, DIM, RESET};
use crate::model::{Entry, FileKind};
use crate::theme::{default_theme, resolve_color_code};

pub fn escaped_name(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    for ch in raw.chars() {
        if ch.is_control() {
            let mut buf = [0u8; 4];
            for byte in ch.encode_utf8(&mut buf).as_bytes() {
                out.push_str(&format!("\\x{:02x}", byte));
            }
        } else {
            out.push(ch);
        }
    }
    out
}

pub fn colorized_name(entry: &Entry, use_color: bool) -> String {
    let escaped = escaped_name(&entry.name);
    let target = entry
        .symlink_target
        .as_ref()
        .map(|target| escaped_name(&format_target_path(target)));
    let theme = default_theme();

    if !use_color {
        return if let Some(target) = target {
            format!("{escaped} -> {target}")
        } else {
            escaped
        };
    }

    if let Some(target) = target {
        let name = match entry.kind {
            FileKind::Symlink if entry.groups_with_directories => {
                colorize_directory_name(&theme, &escaped, entry.color_override.as_deref())
            }
            FileKind::Symlink => colorize_regular_file(&theme, &escaped),
            FileKind::BrokenSymlink => {
                let color = color_for_kind(FileKind::BrokenSymlink);
                if color.is_empty() {
                    escaped
                } else {
                    format!("{color}{escaped}{RESET}")
                }
            }
            _ => {
                let color = color_for_kind(entry.kind);
                if color.is_empty() {
                    escaped
                } else {
                    format!("{color}{escaped}{RESET}")
                }
            }
        };
        return format!("{name} {DIM}-> {target}{RESET}");
    }

    match entry.kind {
        FileKind::Regular => colorize_regular_file(&theme, &escaped),
        FileKind::Directory => {
            colorize_directory_name(&theme, &escaped, entry.color_override.as_deref())
        }
        other => {
            let color = color_for_kind(other);
            if color.is_empty() {
                escaped
            } else {
                format!("{color}{escaped}{RESET}")
            }
        }
    }
}

fn format_target_path(path: &Path) -> String {
    let home = env::var_os("HOME").map(PathBuf::from);
    format_target_path_with_home(path, home.as_deref())
}

fn format_target_path_with_home(path: &Path, home: Option<&Path>) -> String {
    if let Some(home) = home {
        if path == home {
            return "~".to_string();
        }
        if let Ok(rest) = path.strip_prefix(home) {
            if rest.as_os_str().is_empty() {
                return "~".to_string();
            }
            return format!("~/{}", rest.to_string_lossy());
        }
    }
    path.to_string_lossy().into_owned()
}

fn env_style_color_key<'a>(theme: &'a crate::theme::Theme, name: &str) -> Option<&'a str> {
    if name == ".env" || name.starts_with(".env.") {
        if name == ".env.sample" || name.starts_with(".env.sample.") {
            None
        } else {
            theme.known_file_extensions.get("env").copied()
        }
    } else {
        None
    }
}

fn colorize_regular_file(theme: &crate::theme::Theme, name: &str) -> String {
    if let Some(key) = env_style_color_key(theme, name) {
        let color = resolve_color_code(theme, key);
        if !color.is_empty() {
            return format!("{color}{name}{RESET}");
        }
    }

    if let Some(color_key) = theme.known_file_names.get(name) {
        let color = resolve_color_code(&theme, color_key);
        if !color.is_empty() {
            return format!("{color}{name}{RESET}");
        }
    }

    let (base, ext) = match name.rsplit_once('.') {
        Some(parts) => parts,
        None => return name.to_string(),
    };

    let lower_ext = ext.to_ascii_lowercase();
    let mut color_key = theme.known_file_extensions.get(lower_ext.as_str()).copied();

    if color_key.is_none() {
        for (ending, value) in &theme.known_file_extension_endings {
            if lower_ext.ends_with(ending) {
                color_key = Some(value);
                break;
            }
        }
    }

    if let Some(key) = color_key {
        let color = resolve_color_code(&theme, key);
        if !color.is_empty() {
            return format!("{base}{color}.{ext}{RESET}");
        }
    }

    name.to_string()
}

fn colorize_directory_name(
    theme: &crate::theme::Theme,
    name: &str,
    override_color_key: Option<&str>,
) -> String {
    if let Some(key) = override_color_key {
        let color = resolve_color_code(theme, key);
        if !color.is_empty() {
            return format!("{color}{name}{RESET}");
        }
    }

    if let Some(color_key) = theme.known_directory_names.get(name) {
        let color = resolve_color_code(theme, color_key);
        if !color.is_empty() {
            return format!("{color}{name}{RESET}");
        }
    }

    let color = color_for_kind(FileKind::Directory);
    if color.is_empty() {
        name.to_string()
    } else {
        format!("{color}{name}{RESET}")
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};
    use std::time::UNIX_EPOCH;

    use super::{colorized_name, escaped_name, format_target_path_with_home};
    use crate::model::{Entry, FileKind};

    fn regular_file(name: &str) -> Entry {
        Entry {
            path: PathBuf::from(name),
            name: name.to_string(),
            kind: FileKind::Regular,
            groups_with_directories: false,
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
    fn escapes_control_characters() {
        assert_eq!(escaped_name("a\n"), "a\\x0a");
    }

    #[test]
    fn shortens_home_in_symlink_target() {
        let target = Path::new("/Users/max/Documents/file.txt");
        let home = Path::new("/Users/max");
        assert_eq!(
            format_target_path_with_home(target, Some(home)),
            "~/Documents/file.txt"
        );
    }

    #[test]
    fn dims_symlink_arrow_and_target() {
        let entry = Entry {
            path: PathBuf::from("link"),
            name: "link".to_string(),
            kind: FileKind::Symlink,
            groups_with_directories: false,
            color_override: None,
            mode: 0,
            uid: 0,
            gid: 0,
            size: 0,
            modified: UNIX_EPOCH,
            symlink_target: Some(PathBuf::from("/tmp/target")),
        };

        let rendered = colorized_name(&entry, true);
        assert!(rendered.contains("\x1b[2m-> /tmp/target\x1b[0m"));
    }

    #[test]
    fn symlink_uses_regular_file_coloring_rules() {
        let entry = Entry {
            path: PathBuf::from("notes.md"),
            name: "notes.md".to_string(),
            kind: FileKind::Symlink,
            groups_with_directories: false,
            color_override: None,
            mode: 0,
            uid: 0,
            gid: 0,
            size: 0,
            modified: UNIX_EPOCH,
            symlink_target: Some(PathBuf::from("/tmp/target")),
        };

        let rendered = colorized_name(&entry, true);
        assert!(!rendered.contains("\x1b[35m"));
        assert!(rendered.contains("\x1b[38;5;200m"));
    }

    #[test]
    fn directory_symlink_uses_directory_coloring() {
        let entry = Entry {
            path: PathBuf::from("etc"),
            name: "etc".to_string(),
            kind: FileKind::Symlink,
            groups_with_directories: true,
            color_override: None,
            mode: 0,
            uid: 0,
            gid: 0,
            size: 0,
            modified: UNIX_EPOCH,
            symlink_target: Some(PathBuf::from("private/etc")),
        };

        let rendered = colorized_name(&entry, true);
        assert!(rendered.contains("\x1b[34m"));
        assert!(!rendered.contains("\x1b[35m"));
    }

    #[test]
    fn directory_uses_theme_override_color_when_present() {
        let entry = Entry {
            path: PathBuf::from("src"),
            name: "src".to_string(),
            kind: FileKind::Directory,
            groups_with_directories: true,
            color_override: None,
            mode: 0,
            uid: 0,
            gid: 0,
            size: 0,
            modified: UNIX_EPOCH,
            symlink_target: None,
        };

        let rendered = colorized_name(&entry, true);
        assert!(rendered.contains("\x1b[38;5;81m"));
    }

    #[test]
    fn env_file_family_uses_env_coloring() {
        let rendered = colorized_name(&regular_file(".env.local"), true);
        assert!(rendered.contains("\x1b[38;5;13m.env.local\x1b[0m"));
    }

    #[test]
    fn base_env_file_uses_env_coloring() {
        let rendered = colorized_name(&regular_file(".env"), true);
        assert!(rendered.contains("\x1b[38;5;13m.env\x1b[0m"));
    }

    #[test]
    fn env_sample_is_not_forced_to_env_coloring() {
        let rendered = colorized_name(&regular_file(".env.sample"), true);
        assert!(!rendered.contains("\x1b[38;5;13m.env.sample\x1b[0m"));
    }
}
