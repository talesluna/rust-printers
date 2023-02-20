use libc::{c_char, c_int};
use std::{ffi::{CStr, CString}, ptr};

use crate::shared::interface::PlatformPrinterGetters;

/**
 * The CUPS option struct (cups_option_s)
 * https://www.cups.org/doc/cupspm.html#cups_option_s
 */
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CupsOptionT {
    pub name: *mut c_char,
    pub value: *mut c_char,
}

/**
 * The CUPS destination struct (cups_dest_s)
 * https://www.cups.org/doc/cupspm.html#cups_dest_s
 */
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CupsDestT {
    name: *mut c_char,
    instance: *mut c_char,
    is_default: c_int,
    num_options: c_int,
    options: *mut CupsOptionT,
}

/**
 * Implements getters for the cups_dest_s struct
 */
impl CupsDestT {

    /**
     * Returns a string value of an key on cups options (If the key was not found return a empty string)
     */
    pub fn get_option_by_key(&self, key: &str) -> String {
        let mut value = "".to_string();

        for i in 1..self.num_options {
            let option_ptr = unsafe { self.options.offset(i as isize) };
            let option = unsafe { &*option_ptr };

            let name = unsafe { CStr::from_ptr(option.name.clone()) };

            if name.to_string_lossy() == key {
                let value_srt = unsafe { CStr::from_ptr(option.value.clone()) };
                value = value_srt.to_string_lossy().to_string();
                break;
            }
        }

        return value;
    }


}

impl PlatformPrinterGetters for CupsDestT {

    /**
     * Returns the name of the destionation
     */
    fn get_system_name(&self) -> String {
        if self.name.is_null() {
            return "".to_string();
        }

        let c_str = unsafe { CStr::from_ptr(self.name.clone()) };
        return c_str.to_str().unwrap().to_string();
    }

    /**
     * Returns default destination definition
     */
    fn get_is_default(&self) -> c_int {
        return self.is_default;
    }


    /**
     * Returns readable name of dest by "printer-info" option
     */
    fn get_name(&self) -> String {
        return self.get_option_by_key("printer-info");
    }


    /**
     * Returns redeable name of the dest driver by "printer-make-and-model" option
     */
    fn get_marker_and_model(&self) -> String {
        return self.get_option_by_key("printer-make-and-model");
    }

    /**
     * Return if the destination is being shared with other computers
     */
    fn get_is_shared(&self) -> String {
        return self.get_option_by_key("printer-is-shared");
    }

    /**
     * Return the drive version
     */
    fn get_uri(&self) -> String {
        return self.get_option_by_key("device-uri");
    }

    /**
     * Return the location option
     */
    fn get_location(&self) -> String {
        return self.get_option_by_key("printer-location");
    }

    /**
     * Return the state of the CUPS printer
     */
    fn get_state(&self) -> String {
        return self.get_option_by_key("printer-state");
    }

}

#[link(name = "cups")]
extern "C" {
    fn cupsGetDests(dests: *mut *mut CupsDestT) -> c_int;
    fn cupsPrintFile(
        printer_name: *const c_char,
        filename: *const c_char,
        title: *const c_char,
        options: i32,
    ) -> i32;
    // fn cupsFreeDests(num_dests: c_int, dests: *const CupsDestT);
}

/**
 * Returns a vector of CupsDestT (cups_dest_s) struct with all available destinations
 * Using cupsGetDests
 */
pub fn get_dests() -> Vec<&'static CupsDestT> {
    let mut dests_ptr: *mut CupsDestT = ptr::null_mut();
    let dests_count = unsafe { cupsGetDests(&mut dests_ptr) };

    let mut dests: Vec<&CupsDestT> = Vec::new();
    for i in 0..dests_count {
        let dest_ptr = unsafe { dests_ptr.offset(i as isize) };
        let dest = unsafe { &*dest_ptr };

        // Not include printer with null names or duplex shared
        if !dest.name.is_null() && dest.get_is_shared() != "" {
            dests.push(dest);
        }
    }

    return dests;
}

// pub fn free_dests(dests: Vec<&CupsDestT>) {
//     let dest_ptr = dests.as_ptr();
//     unsafe { cupsFreeDests(1, *dest_ptr) };
// }


pub fn print_file(printer_name: &str, file_path: &str) -> bool {
    unsafe {

        let printer_name = CString::new(printer_name).unwrap();
        let filename = CString::new(file_path).unwrap();

        let result = cupsPrintFile(
            printer_name.as_ptr(),
            filename.as_ptr(),
            ptr::null(),
            0,
        );

        return result == 0;
    }
}