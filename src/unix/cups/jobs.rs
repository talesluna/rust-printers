use crate::{
    common::traits::platform::PlatformPrinterJobGetters,
    unix::utils::{
        date::time_t_to_system_time,
        strings::{c_char_to_string, str_to_cstring},
    },
};
use libc::{c_char, c_int, time_t};
use std::ffi::CString;
use std::{slice, time::SystemTime};

#[link(name = "cups")]
extern "C" {

    fn cupsPrintFile(
        printer_name: *const c_char,
        filename: *const c_char,
        title: *const c_char,
        num_options: c_int,
        options: *const CupsOptionT,
    ) -> c_int;

    fn cupsGetJobs(
        jobs: *mut *mut CupsJobsS,
        name: *const c_char,
        // how: c_int,
        myjobs: c_int,
        whichjobs: c_int,
    ) -> c_int;

}

#[derive(Debug)]
#[repr(C)]
struct CupsOptionT {
    name: *const c_char,
    value: *const c_char,
}

#[derive(Debug)]
#[repr(C)]
pub struct CupsJobsS {
    id: c_int,
    dest: *const c_char,
    title: *const c_char,
    user: *const c_char,
    format: *const c_char,
    state: c_int,
    size: c_int,
    priority: c_int,
    completed_time: time_t,
    creation_time: time_t,
    processing_time: time_t,
}

impl PlatformPrinterJobGetters for CupsJobsS {
    fn get_id(&self) -> u64 {
        self.id as u64
    }

    fn get_name(&self) -> String {
        c_char_to_string(self.title)
    }

    fn get_state(&self) -> u64 {
        self.state as u64
    }

    fn get_printer(&self) -> String {
        c_char_to_string(self.dest)
    }

    fn get_media_type(&self) -> String {
        c_char_to_string(self.format)
    }

    fn get_created_at(&self) -> SystemTime {
        time_t_to_system_time(self.creation_time).unwrap()
    }

    fn get_processed_at(&self) -> Option<SystemTime> {
        time_t_to_system_time(self.processing_time)
    }

    fn get_completed_at(&self) -> Option<SystemTime> {
        time_t_to_system_time(self.completed_time)
    }
}

/**
 * Return the printer jobs
 */
pub fn get_printer_jobs(printer_name: &str, active_only: bool) -> Option<&'static [CupsJobsS]> {
    let mut jobs_ptr: *mut CupsJobsS = std::ptr::null_mut();
    let whichjobs = if active_only { 0 } else { -1 };
    let name = str_to_cstring(printer_name);

    unsafe {
        let jobs_count = cupsGetJobs(&mut jobs_ptr, name.as_ptr(), 0, whichjobs);
        if jobs_count > 0 {
            Some(slice::from_raw_parts(jobs_ptr, jobs_count as usize))
        } else {
            None
        }
    }
}

/**
 * Send a file to the printer
 */
pub fn print_file(
    printer_name: &str,
    file_path: &str,
    job_name: Option<&str>,
    options: &[(&str, &str)],
) -> Result<i32, &'static str> {
    unsafe {
        let printer = &str_to_cstring(printer_name);
        let filename = str_to_cstring(file_path);
        let title = str_to_cstring(job_name.unwrap_or(file_path));

        // You ensure those heap-allocated strings:
        // - Stay in memory
        // - Are not dropped
        // - Live through the entire FFI call
        // Then, when print_file() ends, the cstrings Vec is dropped after the FFI call is done, so memory cleanup happens safely.
        let mut cstrings: Vec<CString> = Vec::with_capacity(options.len() * 2);

        let mut cup_opts: Vec<CupsOptionT> = Vec::with_capacity(options.len());

        for (key, value) in options.iter() {
            let c_key = str_to_cstring(key);
            let c_value = str_to_cstring(value);
            cup_opts.push(CupsOptionT {
                name: c_key.as_ptr(),
                value: c_value.as_ptr(),
            });
            cstrings.push(c_key);
            cstrings.push(c_value);
        }

        let result = cupsPrintFile(
            printer.as_ptr(),
            filename.as_ptr(),
            title.as_ptr(),
            cup_opts.len() as c_int,
            if cup_opts.is_empty() {
                std::ptr::null()
            } else {
                cup_opts.as_ptr()
            },
        );

        if result == 0 {
            Err("cupsPrintFile failed")
        } else {
            Ok(result)
        }
    }
}
