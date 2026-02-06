use libc::{c_char, c_int, time_t};
use std::{os::raw::c_void, ptr, slice, time::SystemTime};

use crate::{
    common::{
        base::{errors::PrintersError, options::OptionsCollection},
        traits::platform::PlatformPrinterJobGetters,
    },
    unix::utils::{
        date::time_t_to_system_time,
        strings::{c_char_to_string, str_to_cstring},
    },
};

const CUPS_IPP_OK: c_int = 0x0000;
const CUPS_IPP_TAG_URI: c_int = 0x45;
const CUPS_IPP_TAG_INTEGER: c_int = 0x21;
const CUPS_IPP_TAG_OPERATION: c_int = 0x01;

const CUPS_IPP_OP_HOLD_JOB: c_int = 12;
const CUPS_IPP_OP_CANCEL_JOB: c_int = 8;
const CUPS_IPP_OP_RELEASE_JOB: c_int = 13;
const CUPS_IPP_OP_RESTART_JOB: c_int = 14;

#[link(name = "cups")]
unsafe extern "C" {
    unsafe fn cupsPrintFile(
        printer_name: *const c_char,
        filename: *const c_char,
        title: *const c_char,
        num_options: c_int,
        options: *const CupsOptionT,
    ) -> c_int;

    unsafe fn cupsGetJobs(
        jobs: *mut *mut CupsJobsS,
        name: *const c_char,
        myjobs: c_int,
        whichjobs: c_int,
    ) -> c_int;

    unsafe fn cupsDoRequest(
        http: *mut c_void,
        request: *mut c_void,
        resource: *const c_char,
    ) -> *mut c_void;

    unsafe fn ippAddString(
        req: *mut c_void,
        group: c_int,
        value_tag: c_int,
        name: *const c_char,
        lang: *const c_char,
        value: *const c_char,
    );

    unsafe fn ippAddInteger(
        req: *mut c_void,
        group: c_int,
        value_tag: c_int,
        name: *const c_char,
        value: c_int,
    );

    unsafe fn ippDelete(req: *mut c_void);
    unsafe fn ippNewRequest(op: c_int) -> *mut c_void;
    unsafe fn cupsLastError() -> c_int;
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
        time_t_to_system_time(self.creation_time).unwrap_or(SystemTime::UNIX_EPOCH)
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
    raw_options: &[(&str, &str)],
) -> Result<u64, PrintersError> {
    unsafe {
        let printer = &str_to_cstring(printer_name);
        let filename = str_to_cstring(file_path);
        let title = str_to_cstring(job_name.unwrap_or(file_path));

        let options = OptionsCollection::new(raw_options, |(key, value)| {
            let key = str_to_cstring(key);
            let value = str_to_cstring(value);
            let option = CupsOptionT {
                name: key.as_ptr(),
                value: value.as_ptr(),
            };
            ((key, value), option)
        });

        let result = cupsPrintFile(
            printer.as_ptr(),
            filename.as_ptr(),
            title.as_ptr(),
            options.size as c_int,
            options.as_ptr(),
        );

        if result == 0 {
            Err(PrintersError::print_error("cupsPrintFile"))
        } else {
            Ok(result as u64)
        }
    }
}

/**
 * Send cancel job request to cups
 */
pub fn hold_job(printer_name: &str, job_id: i32) -> bool {
    do_request(printer_name, job_id, CUPS_IPP_OP_HOLD_JOB)
}

/**
 * Send release job request to cups
 */
pub fn release_job(printer_name: &str, job_id: i32) -> bool {
    do_request(printer_name, job_id, CUPS_IPP_OP_RELEASE_JOB)
}

/**
 * Send restart job request to cups
 */
pub fn restart_job(printer_name: &str, job_id: i32) -> bool {
    do_request(printer_name, job_id, CUPS_IPP_OP_RESTART_JOB)
}

/**
 * Send cancel job request to cups
 */
pub fn cancel_job(printer_name: &str, job_id: i32) -> bool {
    do_request(printer_name, job_id, CUPS_IPP_OP_CANCEL_JOB)
}

/**
 * Send request op to cups
 */
fn do_request(printer_name: &str, job_id: i32, op: i32) -> bool {
    unsafe {
        let req = ippNewRequest(op);
        if req.is_null() {
            return false;
        }

        let uri_param = &str_to_cstring("printer-uri");
        let printer_uri =
            str_to_cstring(format!("ipp://localhost/printers/{printer_name}").as_str());

        ippAddString(
            req,
            CUPS_IPP_TAG_OPERATION,
            CUPS_IPP_TAG_URI,
            uri_param.as_ptr(),
            ptr::null(),
            printer_uri.as_ptr(),
        );

        let job_id_param = &str_to_cstring("job-id");
        ippAddInteger(
            req,
            CUPS_IPP_TAG_OPERATION,
            CUPS_IPP_TAG_INTEGER,
            job_id_param.as_ptr(),
            job_id,
        );

        let resource = &str_to_cstring("/");
        let response = cupsDoRequest(ptr::null_mut(), req, resource.as_ptr());
        let status = cupsLastError();
        ippDelete(response);

        status == CUPS_IPP_OK
    }
}
