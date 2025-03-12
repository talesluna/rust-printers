use std::{ffi::CString, mem::transmute_copy};
use libc::{c_char, c_int, c_void};

#[link(name = "kernel32")]
unsafe extern "system" {
    fn FreeLibrary(hModule: *mut c_void) -> c_int;
    fn LoadLibraryA(lpLibFileName: *const c_char) -> *mut c_void;
    fn GetProcAddress(hModule: *mut c_void, lpProcName: *const c_char) -> *mut c_void;
}

pub fn transmute_func<'a, T>(lib:*mut c_void, func_name: &str) -> T {
    unsafe {
        let address = GetProcAddress(lib, CString::new(func_name).unwrap().as_ptr());
        return transmute_copy(&address);
    };
}

pub fn load_lib(dll_path: &str) -> *mut c_void {
    return unsafe {
        let path = CString::new(dll_path).unwrap();
        LoadLibraryA(path.as_ptr())
    }
}

pub fn unload_lib(lib_ptr: *mut c_void) {
    unsafe {
        FreeLibrary(lib_ptr);
    }
}