use crate::{
    common::traits::platform::PlatformPrinterGetters, unix::utils::strings::c_char_to_string,
};
use libc::{c_char, c_int};
use std::{ffi::CString, ptr, slice};

#[link(name = "cups")]
extern "C" {
    fn cupsGetDests(dests: *mut *mut CupsDestT) -> c_int;
    fn cupsFreeDests(num_dests: c_int, dests: *const CupsDestT);
    fn cupsGetOption(
        name: *const c_char,
        num_options: c_int,
        options: *mut CupsOptionT,
    ) -> *const c_char;
}

/**
 * The CUPS option struct (cups_option_s)
 * https://www.cups.org/doc/cupspm.html#cups_option_s
 */
#[derive(Debug)]
#[repr(C)]
pub struct CupsOptionT {
    pub name: *mut c_char,
    pub value: *mut c_char,
}

/**
 * The CUPS destination struct (cups_dest_s)
 * https://www.cups.org/doc/cupspm.html#cups_dest_s
 */
#[derive(Debug)]
#[repr(C)]
pub struct CupsDestT {
    name: *mut c_char,
    instance: *mut c_char,
    is_default: c_int,
    num_options: c_int,
    options: *mut CupsOptionT,
}

impl CupsDestT {
    fn get_option(&self, key: &str) -> String {
        let key = CString::new(key);
        let mut value = "".to_string();

        if !self.options.is_null() && key.is_ok() {
            let option_key = key.unwrap();
            unsafe {
                let option_value =
                    cupsGetOption(option_key.as_ptr(), self.num_options, self.options);
                if !option_value.is_null() {
                    value = c_char_to_string(option_value);
                }
            };
        }

        return value;
    }

    pub fn is_shared_duplex(&self) -> bool {
        return self.num_options == 5;
    }
}

impl PlatformPrinterGetters for CupsDestT {
    fn get_name(&self) -> String {
        return self.get_option("printer-info").trim().to_string();
    }

    fn get_is_default(&self) -> bool {
        return self.is_default == 1;
    }

    fn get_system_name(&self) -> String {
        return c_char_to_string(self.name);
    }

    fn get_marker_and_model(&self) -> String {
        return self.get_option("printer-make-and-model");
    }

    fn get_is_shared(&self) -> bool {
        return self.get_option("printer-is-shared") == "true";
    }

    fn get_uri(&self) -> String {
        return self.get_option("printer-uri-supported");
    }

    fn get_location(&self) -> String {
        return self.get_option("printer-location");
    }

    fn get_state(&self) -> u64 {
        return self.get_option("printer-state").parse::<u64>().unwrap_or_default();
    }

    fn get_state_reasons(&self) -> Vec<String> {
        return self.get_option("printer-state-reasons").split(",").map(|v| v.to_string()).collect();
    }

    fn get_port_name(&self) -> String {
        return self.get_option("device-uri");
    }

    fn get_processor(&self) -> String {
        return "".to_string();
    }

    fn get_description(&self) -> String {
        return "".to_string();
    }

    fn get_data_type(&self) -> String {
        return self.get_option("media");
    }
}

/**
 * Returns a vector of CupsDestT (cups_dest_s) struct with all available destinations
 * Using cupsGetDests
 */
pub fn get_dests() -> Option<&'static [CupsDestT]> {
    unsafe {
        let mut dests_ptr: *mut CupsDestT = ptr::null_mut();
        let dests_count: i32 = cupsGetDests(&mut dests_ptr);
        return if dests_count > 0 {
            Some(slice::from_raw_parts(dests_ptr, dests_count as usize))
        } else {
            None
        };
    }
}

/**
 * Free dests memory
 */
pub fn free(dests: &'static [CupsDestT]) {
    if dests.len() > 0 {
        unsafe {
            cupsFreeDests(dests.len() as i32, dests.as_ptr() as *mut CupsDestT);
        }
    }
}
