use crate::{
    common::traits::platform::PlatformPrinterJobGetters,
    unix::utils::{
        date::time_t_to_system_time,
        strings::{c_char_to_string, str_to_cstring},
    },
};
use libc::{c_char, c_int, time_t};
use std::{slice, time::SystemTime};

#[link(name = "cups")]
extern "C" {

    fn cupsPrintFile(
        printer_name: *const c_char,
        filename: *const c_char,
        title: *const c_char,
        options: i32,
    ) -> i32;

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
) -> Result<(), &'static str> {
    unsafe {
        let printer = &str_to_cstring(printer_name);
        let filename = str_to_cstring(file_path);
        let title = str_to_cstring(job_name.unwrap_or(file_path));

        let result = cupsPrintFile(printer.as_ptr(), filename.as_ptr(), title.as_ptr(), 0);
        if result == 0 {
            Err("cupsPrintFile failed")
        } else {
            Ok(())
        }
    }
}
