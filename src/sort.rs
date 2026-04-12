use std::cmp::Ordering;

use crate::model::{Entry, FileKind, SortMode};

pub fn sort_entries(entries: &mut [Entry], sort_mode: SortMode, reverse: bool) {
    entries.sort_by(|a, b| compare_entries(a, b, sort_mode));
    if reverse {
        entries.reverse();
    }
}

fn compare_entries(a: &Entry, b: &Entry, sort_mode: SortMode) -> Ordering {
    let a_dir = a.kind == FileKind::Directory;
    let b_dir = b.kind == FileKind::Directory;
    let dir_cmp = b_dir.cmp(&a_dir);
    if dir_cmp != Ordering::Equal {
        return dir_cmp;
    }

    let primary = match sort_mode {
        SortMode::Version => natural_cmp(&a.name, &b.name),
        SortMode::Name => a.name.cmp(&b.name),
        SortMode::Time => b.modified.cmp(&a.modified),
        SortMode::Size => b.size.cmp(&a.size),
        SortMode::Extension => extension_of(&a.name)
            .cmp(&extension_of(&b.name))
            .then_with(|| a.name.cmp(&b.name)),
    };

    primary.then_with(|| a.name.cmp(&b.name))
}

fn extension_of(name: &str) -> &str {
    name.rsplit_once('.').map(|(_, ext)| ext).unwrap_or("")
}

fn natural_cmp(a: &str, b: &str) -> Ordering {
    let mut ai = 0;
    let mut bi = 0;
    let ab = a.as_bytes();
    let bb = b.as_bytes();

    while ai < ab.len() && bi < bb.len() {
        let ac = ab[ai];
        let bc = bb[bi];

        if ac.is_ascii_digit() && bc.is_ascii_digit() {
            let (an, anext) = read_number(ab, ai);
            let (bn, bnext) = read_number(bb, bi);
            match an.cmp(&bn) {
                Ordering::Equal => {
                    ai = anext;
                    bi = bnext;
                }
                ord => return ord,
            }
            continue;
        }

        let ord = ac.to_ascii_lowercase().cmp(&bc.to_ascii_lowercase());
        if ord != Ordering::Equal {
            return ord;
        }
        ai += 1;
        bi += 1;
    }

    ab.len().cmp(&bb.len())
}

fn read_number(bytes: &[u8], mut index: usize) -> (u64, usize) {
    let mut value: u64 = 0;
    while index < bytes.len() && bytes[index].is_ascii_digit() {
        value = value
            .saturating_mul(10)
            .saturating_add((bytes[index] - b'0') as u64);
        index += 1;
    }
    (value, index)
}

#[cfg(test)]
mod tests {
    use super::natural_cmp;
    use std::cmp::Ordering;

    #[test]
    fn natural_sort_works() {
        assert_eq!(natural_cmp("file2", "file10"), Ordering::Less);
        assert_eq!(natural_cmp("v12", "v3"), Ordering::Greater);
    }
}
