#[cfg(target_family = "unix")]
use std::ffi::{CStr, CString};

#[cfg(target_family = "unix")]
use libc::c_char;

#[cfg(target_family = "windows")]
use libc::wchar_t;

#[cfg(target_family = "unix")]
pub fn c_char_to_string(value: *const c_char) -> String {
    return if !value.is_null() {
        let cstr = unsafe { CStr::from_ptr(value) };
        cstr.to_string_lossy().to_string()
    } else {
        "".to_string()
    };
}

#[cfg(target_family = "unix")]
pub fn str_to_cstring(value: &str) -> CString {
    let c_string = CString::new(value);
    return c_string.unwrap();
}

#[cfg(target_family = "windows")]
pub fn str_to_wide_string(value: &str) -> Vec<u16> {
    return value.encode_utf16().chain(Some(0)).collect();
}

#[cfg(target_family = "windows")]
pub fn wchar_t_to_string(value: *const wchar_t) -> String {
    if value.is_null() {
        return "".to_string();
    }

    let mut vec: Vec<u16> = Vec::new();
    let mut i = 0;
    unsafe {
        while *value.offset(i) != 0 {
            vec.push(*value.offset(i) as u16);
            i += 1;
        }
    }
    return String::from_utf16_lossy(&vec);
}
