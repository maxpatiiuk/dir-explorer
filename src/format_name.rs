use crate::color::{color_for_kind, RESET};
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
    let display = if let Some(target) = &entry.symlink_target {
        format!("{escaped} -> {}", escaped_name(&target.to_string_lossy()))
    } else {
        escaped
    };

    if !use_color {
        return display;
    }

    match entry.kind {
        FileKind::Regular => colorize_regular_file(&display),
        other => {
            let color = color_for_kind(other);
            if color.is_empty() {
                display
            } else {
                format!("{color}{display}{RESET}")
            }
        }
    }
}

fn colorize_regular_file(name: &str) -> String {
    let theme = default_theme();

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

#[cfg(test)]
mod tests {
    use super::escaped_name;

    #[test]
    fn escapes_control_characters() {
        assert_eq!(escaped_name("a\n"), "a\\x0a");
    }
}
