use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortMode {
    Version,
    Name,
    Time,
    Size,
    Extension,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorWhen {
    Auto,
    Always,
    Never,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    Default,
    Long,
    Zero,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileKind {
    Regular,
    Executable,
    Directory,
    Symlink,
    BrokenSymlink,
    Pipe,
    Socket,
    BlockDevice,
    CharDevice,
    Other,
}

#[derive(Debug, Clone)]
pub struct Options {
    pub show_all: bool,
    pub almost_all: bool,
    pub reverse: bool,
    pub recursive: bool,
    pub human_readable: bool,
    pub sort_mode: SortMode,
    pub color_when: ColorWhen,
    pub view_mode: ViewMode,
    pub paths: Vec<PathBuf>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            show_all: false,
            almost_all: true,
            reverse: false,
            recursive: false,
            human_readable: true,
            sort_mode: SortMode::Version,
            color_when: ColorWhen::Auto,
            view_mode: ViewMode::Default,
            paths: vec![PathBuf::from(".")],
        }
    }
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub path: PathBuf,
    pub name: String,
    pub kind: FileKind,
    pub mode: u32,
    pub nlink: u64,
    pub uid: u32,
    pub gid: u32,
    pub size: u64,
    pub modified: SystemTime,
    pub symlink_target: Option<PathBuf>,
}
