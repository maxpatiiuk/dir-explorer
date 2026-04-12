use std::path::PathBuf;

use crate::model::{ColorWhen, Options, SortMode, ViewMode};

pub fn parse_args<I>(args: I) -> Result<Options, String>
where
    I: IntoIterator<Item = String>,
{
    let mut options = Options::default();
    let mut paths: Vec<PathBuf> = Vec::new();

    for arg in args.into_iter().skip(1) {
        if arg == "--help" || arg == "-?" {
            return Err(help_text());
        }

        if let Some(value) = arg.strip_prefix("--sort=") {
            options.sort_mode = match value {
                "version" => SortMode::Version,
                "name" => SortMode::Name,
                "time" => SortMode::Time,
                "size" => SortMode::Size,
                "extension" => SortMode::Extension,
                _ => return Err(format!("Unsupported --sort value: {value}")),
            };
            continue;
        }

        if let Some(value) = arg.strip_prefix("--color=") {
            options.color_when = match value {
                "always" => ColorWhen::Always,
                "auto" => ColorWhen::Auto,
                "never" => ColorWhen::Never,
                _ => return Err(format!("Unsupported --color value: {value}")),
            };
            continue;
        }

        match arg.as_str() {
            "--reverse" => {
                options.reverse = true;
                continue;
            }
            "--recursive" => {
                options.recursive = true;
                continue;
            }
            _ => {}
        }

        if arg.starts_with('-') && arg.len() > 1 {
            for short in arg[1..].chars() {
                match short {
                    'a' => {
                        options.show_all = true;
                        options.almost_all = false;
                    }
                    'A' => {
                        options.almost_all = true;
                        options.show_all = false;
                    }
                    'l' => options.view_mode = ViewMode::Long,
                    '0' => options.view_mode = ViewMode::Zero,
                    'H' => options.human_readable = false,
                    'r' => options.reverse = true,
                    'R' => options.recursive = true,
                    't' => options.sort_mode = SortMode::Time,
                    _ => return Err(format!("Unsupported flag: -{short}")),
                }
            }
        } else {
            paths.push(PathBuf::from(arg));
        }
    }

    if !paths.is_empty() {
        options.paths = paths;
    }

    Ok(options)
}

pub fn help_text() -> String {
    [
        "Usage: l [options] [paths...]",
        "",
        "Defaults: -A, --sort=version, long-iso time, dirs first",
        "",
        "Flags:",
        "  -a                show all entries including . and ..",
        "  -A                show almost all entries (default)",
        "  -l                detailed mode: mode links owner group size time name",
        "  -0                compact mode: only names",
        "  -H                show raw bytes instead of human-readable sizes",
        "  -r, --reverse     reverse final sort order",
        "  -R, --recursive   recursive tree output (does not recurse into symlinks)",
        "  -t                sort by modified time",
        "      --sort=...    version|name|time|size|extension",
        "      --color=...   always|auto|never",
        "      --help        show this help",
    ]
    .join("\n")
}

#[cfg(test)]
mod tests {
    use super::parse_args;
    use crate::model::{SortMode, ViewMode};

    #[test]
    fn parses_long_mode() {
        let options = parse_args(["l".into(), "-l".into()]).unwrap();
        assert_eq!(options.view_mode, ViewMode::Long);
    }

    #[test]
    fn parses_zero_mode() {
        let options = parse_args(["l".into(), "-0".into()]).unwrap();
        assert_eq!(options.view_mode, ViewMode::Zero);
    }

    #[test]
    fn parses_time_sort() {
        let options = parse_args(["l".into(), "-t".into()]).unwrap();
        assert_eq!(options.sort_mode, SortMode::Time);
    }
}
