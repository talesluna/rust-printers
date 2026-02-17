#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use libc::{c_int, c_ulong, c_ushort, c_void, wchar_t};
use std::{ffi::c_char, ptr, slice};

use crate::{
    common::{base::{errors::PrintersError, job::PrinterJobOptions}, traits::platform::PlatformPrinterJobGetters},
    windows::utils::{
        date::{calculate_system_time, get_current_epoch},
        memory::alloc_s,
        strings::{str_to_wide_string, wchar_t_to_string},
    },
};

#[link(name = "winspool")]
unsafe extern "system" {
    fn OpenPrinterW(
        pPrinterName: *const wchar_t,
        phPrinter: *mut *mut c_void,
        pDefault: *mut c_void,
    ) -> c_int;
    fn StartDocPrinterW(
        hPrinter: *mut c_void,
        Level: c_ulong,
        pDocInfo: *const DOC_INFO_1W,
    ) -> c_ulong;
    fn StartPagePrinter(hPrinter: *mut c_void) -> c_int;
    fn WritePrinter(
        hPrinter: *mut c_void,
        pBuf: *mut c_void,
        cbBuf: c_ulong,
        pcWritten: *mut c_ulong,
    ) -> c_int;
    fn EndPagePrinter(hPrinter: *mut c_void) -> c_int;
    fn EndDocPrinter(hPrinter: *mut c_void) -> c_int;
    fn ClosePrinter(hPrinter: *mut c_void) -> c_int;
    fn EnumJobsW(
        hPrinter: *mut c_void,
        firstJob: c_ulong,
        noJobs: c_ulong,
        level: c_ulong,
        pJob: *mut c_void,
        cbBuf: c_ulong,
        pcbNeeded: *mut c_ulong,
        pcReturned: *mut c_ulong,
    ) -> c_int;
    fn SetJobW(
        hPrinter: *mut c_void,
        JobId: c_ulong,
        Level: c_ulong,
        pJob: *mut c_char,
        Command: c_ulong,
    ) -> c_int;
}

#[repr(C)]
struct DOC_INFO_1W {
    pDocName: *mut wchar_t,
    pOutputFile: *mut wchar_t,
    pDatatype: *mut wchar_t,
}

/**
 * Specifies a date and time, using individual members for the month, day, year, weekday, hour, minute, second, and millisecond.
 * https://learn.microsoft.com/en-us/windows/win32/api/minwinbase/ns-minwinbase-systemtime
 */
#[derive(Debug, Clone)]
#[repr(C)]
pub struct SYSTEMTIME {
    pub wYear: c_ushort,
    pub wMonth: c_ushort,
    pub wDayOfWeek: c_ushort,
    pub wDay: c_ushort,
    pub wHour: c_ushort,
    pub wMinute: c_ushort,
    pub wSecond: c_ushort,
    pub wMilliseconds: c_ushort,
}

/**
 * The JOB_INFO_1 structure specifies print-job information.
 * https://learn.microsoft.com/en-us/windows/win32/printdocs/job-info-1
 */
#[derive(Debug, Clone)]
#[repr(C)]
pub struct JOB_INFO_1W {
    JobId: c_ulong,
    pPrinterName: *mut wchar_t,
    pMachineName: *mut wchar_t,
    pUserName: *mut wchar_t,
    pDocument: *mut wchar_t,
    pDatatype: *mut wchar_t,
    pStatus: *mut wchar_t,
    Status: c_ulong,
    Priority: c_ulong,
    Position: c_ulong,
    TotalPages: c_ulong,
    PagesPrinted: c_ulong,
    Submitted: SYSTEMTIME,
}

impl PlatformPrinterJobGetters for JOB_INFO_1W {
    fn get_id(&self) -> u64 {
        self.JobId.into()
    }

    fn get_name(&self) -> String {
        wchar_t_to_string(self.pDocument)
    }

    fn get_state(&self) -> u64 {
        self.Status.into()
    }

    fn get_printer(&self) -> String {
        wchar_t_to_string(self.pPrinterName)
    }

    fn get_media_type(&self) -> String {
        wchar_t_to_string(self.pDatatype)
    }

    fn get_created_at(&self) -> std::time::SystemTime {
        calculate_system_time(
            self.Submitted.wYear,
            self.Submitted.wMonth,
            self.Submitted.wDay,
            self.Submitted.wHour,
            self.Submitted.wMinute,
            self.Submitted.wSecond,
            self.Submitted.wMilliseconds,
        )
    }

    fn get_processed_at(&self) -> Option<std::time::SystemTime> {
        Some(self.get_created_at())
    }

    fn get_completed_at(&self) -> Option<std::time::SystemTime> {
        Some(self.get_created_at())
    }
}

/**
 * Open printer utility
 */
fn open_printer(printer_name: &str) -> Result<*mut c_void, PrintersError> {
    let printer_name = str_to_wide_string(printer_name);
    let mut printer_handle: *mut c_void = ptr::null_mut();

    if unsafe {
        OpenPrinterW(
            printer_name.as_ptr() as *const wchar_t,
            &mut printer_handle,
            ptr::null_mut(),
        )
    } == 0 {
        Err(PrintersError::job_error("OpenPrinterW failed"))
    } else {
        Ok(printer_handle)
    }
}

/**
 * Print a buffer as RAW datatype with winspool WritePrinterx
 */
pub fn print_buffer(
    printer_name: &str,
    buffer: &[u8],
    options: &PrinterJobOptions,
) -> Result<u64, PrintersError> {
    unsafe {
        let printer_handle = open_printer(printer_name)?;

        let copies = options.copies.clone().unwrap_or(1);
        let data_type = options.data_type.clone().unwrap_or("RAW".into());

        let mut p_data_type = str_to_wide_string(data_type.as_str());
        let mut p_doc_name =
            str_to_wide_string(options.name.clone().unwrap_or(get_current_epoch().to_string()).as_str());

        let doc_info = DOC_INFO_1W {
            pDocName: p_doc_name.as_mut_ptr() as *mut wchar_t,
            pDatatype: p_data_type.as_mut_ptr() as *mut wchar_t,
            pOutputFile: ptr::null_mut(),
        };

        let job_id = StartDocPrinterW(printer_handle, 1, &doc_info);
        if job_id == 0 {
            ClosePrinter(printer_handle);
            return Err(PrintersError::job_error("StartDocPrinterW failed"));
        }

        for _ in 0..copies {
            if StartPagePrinter(printer_handle) != 0 {
                let mut bytes_written: c_ulong = 0;
                WritePrinter(
                    printer_handle,
                    buffer.as_ptr() as *mut c_void,
                    buffer.len() as c_ulong,
                    &mut bytes_written,
                );
                EndPagePrinter(printer_handle);
            }
        }

        EndDocPrinter(printer_handle);
        ClosePrinter(printer_handle);

        Ok(job_id as u64)
    }
}

/**
 * Retrieve print jobs of a specific printer with EnumJobsW
 */
pub fn enum_printer_jobs(printer_name: &str) -> Result<&'static [JOB_INFO_1W], PrintersError> {
    let printer_handle = open_printer(printer_name)?;

    let mut enum_result = 0;
    let mut buffer_ptr: *mut JOB_INFO_1W = ptr::null_mut();
    let mut jobs_count: c_ulong = 0;
    let mut bytes_needed: c_ulong = 0;

    for _ in 0..2 {
        enum_result = unsafe {
            EnumJobsW(
                printer_handle,
                0,
                0xFFFFFFFF,
                1,
                buffer_ptr as *mut c_void,
                bytes_needed,
                &mut bytes_needed,
                &mut jobs_count,
            )
        };

        if enum_result != 0 || bytes_needed == 0 {
            break;
        }

        buffer_ptr = alloc_s::<JOB_INFO_1W>(bytes_needed);
    }

    unsafe { ClosePrinter(printer_handle) };

    if enum_result == 0 {
        return Err(PrintersError::job_error("EnumJobsW failed"));
    }

    Ok(if jobs_count > 0 {
        unsafe { slice::from_raw_parts(buffer_ptr as *const JOB_INFO_1W, jobs_count as usize) }
    } else {
        &[]
    })
}

/**
 * Change job state
 */
pub fn set_job_state(printer_name: &str, command: u64, job_id: u64) -> Result<(), PrintersError> {
    unsafe {
        let printer_handle = open_printer(printer_name)?;

        let result = SetJobW(
            printer_handle,
            job_id as c_ulong,
            0,
            ptr::null_mut(),
            command as c_ulong,
        );

        ClosePrinter(printer_handle);

        if result == 0 {
            Err(PrintersError::job_error("SetJobW failed"))
        } else {
            Ok(())
        }
    }
}
