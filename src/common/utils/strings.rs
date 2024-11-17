use std::{
    ffi::{CStr, CString},
    ptr,
};

use libc::{c_char, wchar_t};

pub fn c_char_to_string(value: *const c_char) -> String {
    return if !value.is_null() {
        let cstr = unsafe { CStr::from_ptr(value) };
        cstr.to_string_lossy().to_string()
    } else {
        "".to_string()
    };
}

pub fn str_to_c_char_ptr(value: &str) -> *const c_char {
    let c_string = CString::new(value);
    return if c_string.is_ok() {
        let result = c_string.unwrap();
        result.as_ptr()
    } else {
        ptr::null()
    };
}

pub fn str_to_wchar_t_ptr(value: &str) -> *mut wchar_t {
    let mut wide: Vec<u16> = value.encode_utf16().chain(Some(0)).collect();
    return wide.as_mut_ptr() as *mut wchar_t;
}

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
