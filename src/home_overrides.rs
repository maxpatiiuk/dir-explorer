use std::path::Path;
use std::{env, fs};

use crate::model::{Entry, FileKind, Options, ViewMode};

/// Applies home-directory-specific filtering and color overrides.
///
/// Returns the number of entries hidden by these rules.
pub fn apply_home_directory_overrides(
    path: &Path,
    options: &Options,
    entries: &mut Vec<Entry>,
) -> usize {
    if options.view_mode == ViewMode::Long || !is_home_directory(path) {
        return 0;
    }

    let mut filtered = Vec::with_capacity(entries.len());
    let mut hidden_count = 0usize;

    for mut entry in entries.drain(..) {
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

        if !matches!(entry.name.as_str(), "d" | "g" | "j" | "e") {
            entry.color_override = Some("gray".to_string());
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
