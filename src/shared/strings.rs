use libc::{c_char, c_void, free, wchar_t};
use std::ffi::{CStr, CString};
use std::ptr;

pub fn string_from_wchar_t(s: *mut wchar_t) -> String {
    if s.is_null() {
        return "".to_string()
    }

    let mut vec: Vec<u16> = Vec::new();
    let mut i = 0;
    unsafe {
        while *s.offset(i) != 0 {
            vec.push(*s.offset(i) as u16);
            i += 1;
        }
    }
    return String::from_utf16_lossy(&vec);
}

// pub fn string_from_c_char(s: *mut c_char) -> String {
//     if s.is_null() {
//         return "".to_string()
//     }

//     let c_str = unsafe { CStr::from_ptr(s.clone()) };
//     return c_str.to_str().unwrap().to_string();
// }
