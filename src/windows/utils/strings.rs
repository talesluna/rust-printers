use libc::wchar_t;

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

pub fn str_to_wide_string(value: &str) -> Vec<u16> {
    return value.encode_utf16().chain(Some(0)).collect();
}

pub fn str_to_wide_string_ptr(value: &str) -> *const u16 {
    let result = str_to_wide_string(value);
    return result.as_ptr();
}
