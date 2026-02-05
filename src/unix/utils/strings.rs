use libc::c_char;
use std::ffi::{CStr, CString};

pub fn c_char_to_string(value: *const c_char) -> String {
    if !value.is_null() {
        let cstr = unsafe { CStr::from_ptr(value) };
        cstr.to_string_lossy().to_string()
    } else {
        "".to_string()
    }
}

pub fn str_to_cstring(value: &str) -> CString {
    let c_string = CString::new(value);
    c_string.unwrap_or_default()
}
