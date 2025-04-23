use libc::c_ushort;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn is_leap_year(year: c_ushort) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn days_in_months(year: c_ushort, month: c_ushort) -> c_ushort {
    if month == 2 {
        if is_leap_year(year) { 29 } else { 28 }
    } else if month == 4 || month == 6 || month == 9 || month == 11 {
        30
    } else {
        31
    }
}

pub fn calculate_system_time(
    year: c_ushort,
    month: c_ushort,
    day: c_ushort,
    hour: c_ushort,
    minute: c_ushort,
    second: c_ushort,
    milliseconds: c_ushort,
) -> SystemTime {
    let mut total_days: c_ushort = day - 1;

    for year in 1970..year {
        total_days += if is_leap_year(year) { 366 } else { 365 };
    }

    for month in 0..(month - 1) {
        total_days += days_in_months(year, month);
    }

    let total_seconds = (total_days as u64 * 24 * 60 * 60)
        + (hour as u64 * 60 * 60)
        + (minute as u64 * 60)
        + (second as u64);

    UNIX_EPOCH + Duration::new(total_seconds.into(), milliseconds as u32 * 1_000_000)
}

pub fn get_current_epoch() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
