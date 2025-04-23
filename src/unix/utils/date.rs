use libc::time_t;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn time_t_to_system_time(value: time_t) -> Option<SystemTime> {
    return if value > 0 {
        let time = UNIX_EPOCH + Duration::from_secs(value as u64);
        Some(time)
    } else {
        None
    };
}
