use std::ffi::CStr;

use libc::{getgrgid, getpwuid};
use time::format_description::parse;
use time::{OffsetDateTime, UtcOffset};

pub fn format_size(size: u64, human: bool) -> String {
    if !human {
        return size.to_string();
    }

    const UNITS: [&str; 5] = ["", "k", "m", "g", "t"];
    let mut value = size as f64;
    let mut index = 0;
    while value >= 1024.0 && index < UNITS.len() - 1 {
        value /= 1024.0;
        index += 1;
    }

    if index == 0 {
        format!("{}", value as u64)
    } else {
        format!("{value:.1}{}", UNITS[index])
    }
}

pub fn format_mode_octal(mode: u32) -> String {
    format!("{:o}", mode & 0o7777)
}

pub fn format_time_long_iso(system_time: std::time::SystemTime) -> String {
    let unix = match system_time.duration_since(std::time::UNIX_EPOCH) {
        Ok(duration) => duration.as_secs() as i64,
        Err(_) => 0,
    };

    let offset = UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC);
    let datetime = OffsetDateTime::from_unix_timestamp(unix)
        .unwrap_or(OffsetDateTime::UNIX_EPOCH)
        .to_offset(offset);
    let format = parse("[year]-[month]-[day] [hour]:[minute]").unwrap();
    datetime
        .format(&format)
        .unwrap_or_else(|_| "1970-01-01 00:00".to_string())
}

pub fn resolve_user(uid: u32) -> String {
    unsafe {
        let ptr = getpwuid(uid);
        if ptr.is_null() {
            return uid.to_string();
        }
        let name = CStr::from_ptr((*ptr).pw_name);
        name.to_string_lossy().into_owned()
    }
}

pub fn resolve_group(gid: u32) -> String {
    unsafe {
        let ptr = getgrgid(gid);
        if ptr.is_null() {
            return gid.to_string();
        }
        let name = CStr::from_ptr((*ptr).gr_name);
        name.to_string_lossy().into_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::{format_mode_octal, format_size};

    #[test]
    fn human_size() {
        assert_eq!(format_size(123, true), "123");
        assert_eq!(format_size(2048, true), "2.0k");
    }

    #[test]
    fn octal_mode_without_leading_zero() {
        assert_eq!(format_mode_octal(0o100755), "755");
    }
}
