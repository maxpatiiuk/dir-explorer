use std::ffi::OsStr;
use std::fs;
use std::os::unix::fs::{FileTypeExt, MetadataExt};
use std::path::Path;

use crate::model::{Entry, FileKind, Options, ViewMode};

pub fn collect_entries(path: &Path, options: &Options) -> Result<Vec<Entry>, String> {
    let read_dir = fs::read_dir(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut entries = Vec::new();

    for dir_entry in read_dir {
        let dir_entry = dir_entry.map_err(|e| format!("{}: {e}", path.display()))?;
        let name_os = dir_entry.file_name();

        if should_skip(&name_os, options) {
            continue;
        }

        let full_path = dir_entry.path();
        entries.push(collect_entry_for_path(
            &full_path,
            Some(name_os.to_string_lossy().into_owned()),
        )?);
    }

    Ok(entries)
}

pub fn collect_entry_for_path(path: &Path, display_name: Option<String>) -> Result<Entry, String> {
    let symlink_meta =
        fs::symlink_metadata(path).map_err(|e| format!("{}: {e}", path.display()))?;

    let mut kind = classify_kind(&symlink_meta);
    let symlink_target = if kind == FileKind::Symlink {
        Some(fs::read_link(path).map_err(|e| format!("{}: {e}", path.display()))?)
    } else {
        None
    };

    let mut groups_with_directories = kind == FileKind::Directory;
    if kind == FileKind::Symlink {
        match fs::metadata(path) {
            Ok(target_meta) => {
                if target_meta.is_dir() {
                    groups_with_directories = true;
                }
            }
            Err(_) => {
                kind = FileKind::BrokenSymlink;
            }
        }
    }

    let name = match display_name {
        Some(name) => name,
        None => path
            .file_name()
            .map(|value| value.to_string_lossy().into_owned())
            .unwrap_or_else(|| path.to_string_lossy().into_owned()),
    };

    Ok(Entry {
        path: path.to_path_buf(),
        name,
        kind,
        groups_with_directories,
        color_override: None,
        mode: symlink_meta.mode(),
        uid: symlink_meta.uid(),
        gid: symlink_meta.gid(),
        size: symlink_meta.size(),
        modified: symlink_meta.modified().unwrap_or(std::time::UNIX_EPOCH),
        symlink_target,
    })
}

fn should_skip(name: &OsStr, options: &Options) -> bool {
    let name = name.to_string_lossy();
    if options.view_mode != ViewMode::Long && name == ".DS_Store" {
        return true;
    }
    if options.show_all {
        return false;
    }
    if options.almost_all {
        return name == "." || name == "..";
    }
    false
}

fn classify_kind(meta: &fs::Metadata) -> FileKind {
    let file_type = meta.file_type();
    let mode = meta.mode();

    if file_type.is_dir() {
        return FileKind::Directory;
    }
    if file_type.is_symlink() {
        return FileKind::Symlink;
    }
    if file_type.is_file() {
        if mode & 0o111 != 0 {
            return FileKind::Executable;
        }
        return FileKind::Regular;
    }

    if file_type.is_fifo() {
        return FileKind::Pipe;
    }
    if file_type.is_socket() {
        return FileKind::Socket;
    }
    if file_type.is_block_device() {
        return FileKind::BlockDevice;
    }
    if file_type.is_char_device() {
        return FileKind::CharDevice;
    }

    FileKind::Other
}

pub fn is_directory_for_recursion(entry: &Entry) -> bool {
    entry.kind == FileKind::Directory
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::fs::collect_entries;
    use crate::model::{Options, ViewMode};

    fn make_temp_dir() -> PathBuf {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("dir_explorer_test_{seed}"));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn almost_all_includes_dotfiles() {
        let dir = make_temp_dir();
        fs::write(dir.join(".hidden"), b"x").unwrap();
        fs::write(dir.join("visible"), b"y").unwrap();

        let options = Options::default();
        let entries = collect_entries(&dir, &options).unwrap();
        let names: Vec<String> = entries.into_iter().map(|entry| entry.name).collect();

        assert!(names.iter().any(|name| name == ".hidden"));
        assert!(names.iter().any(|name| name == "visible"));

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn ds_store_hidden_in_non_long_mode() {
        let dir = make_temp_dir();
        fs::write(dir.join(".DS_Store"), b"x").unwrap();
        fs::write(dir.join("visible"), b"y").unwrap();

        let options = Options::default();
        let entries = collect_entries(&dir, &options).unwrap();
        let names: Vec<String> = entries.into_iter().map(|entry| entry.name).collect();

        assert!(!names.iter().any(|name| name == ".DS_Store"));
        assert!(names.iter().any(|name| name == "visible"));

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn ds_store_visible_in_long_mode() {
        let dir = make_temp_dir();
        fs::write(dir.join(".DS_Store"), b"x").unwrap();

        let mut options = Options::default();
        options.view_mode = ViewMode::Long;
        let entries = collect_entries(&dir, &options).unwrap();
        let names: Vec<String> = entries.into_iter().map(|entry| entry.name).collect();

        assert!(names.iter().any(|name| name == ".DS_Store"));

        fs::remove_dir_all(dir).unwrap();
    }
}
