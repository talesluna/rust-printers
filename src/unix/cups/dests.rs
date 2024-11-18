use std::{ffi::CString, ptr, slice, iter::FromIterator};

use libc::{c_char, c_int};

use crate::common::{traits::platform::PlatformPrinterGetters, utils};

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
                    value = utils::strings::c_char_to_string(option_value);
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
    /**
     * Returns readable name of dest by "printer-info" option
     */
    fn get_name(&self) -> String {
        return self.get_option("printer-info").trim().to_string();
    }

    /**
     * Returns default destination definition
     */
    fn get_is_default(&self) -> bool {
        return self.is_default == 1;
    }

    /**
     * Returns the name of the destination
     */
    fn get_system_name(&self) -> String {
        return utils::strings::c_char_to_string(self.name);
    }

    /**
     * Returns readable name of the dest driver by "printer-make-and-model" option
     */
    fn get_marker_and_model(&self) -> String {
        return self.get_option("printer-make-and-model");
    }

    /**
     * Return if the destination is being shared with other computers
     */
    fn get_is_shared(&self) -> bool {
        return self.get_option("printer-is-shared") == "true";
    }

    /**
     * Return the printer URI
     */
    fn get_uri(&self) -> String {
        return self.get_option("printer-uri-supported");
    }

    /**
     * Return the location option
     */
    fn get_location(&self) -> String {
        return self.get_option("printer-location");
    }

    /**
     * Return the state of the CUPS printer
     */
    fn get_state(&self) -> String {
        return self.get_option("printer-state");
    }

    /**
     * Return the printer port name
     */
    fn get_port_name(&self) -> String {
        return self.get_option("device-uri");
    }

    /**
     * Return the printer processor name
     */
    fn get_processor(&self) -> String {
        return "".to_string();
    }

    /**
     * Return the printer comment
     */
    fn get_description(&self) -> String {
        return "".to_string();
    }

    /**
     * Return the printer data type
     */
    fn get_data_type(&self) -> String {
        return self.get_option("media");
    }
}

/**
 * Returns a vector of CupsDestT (cups_dest_s) struct with all available destinations
 * Using cupsGetDests
 */
pub fn get_dests() -> &'static [CupsDestT] {
    unsafe {
        let mut dests_ptr: *mut CupsDestT = ptr::null_mut();
        let dests_count: i32 = cupsGetDests(&mut dests_ptr);
        return slice::from_raw_parts(dests_ptr, dests_count as usize);
    }
}

/**
 * Free dests memory
 */
pub fn free(dests: &'static [CupsDestT]) {
    if dests.len() > 0 {
        unsafe {
            let ptr = Box::from_iter(dests).as_ptr();
            cupsFreeDests(dests.len() as i32, *ptr);
        }
    }
}
