#![allow(non_snake_case)]

use libc::{c_int, c_ulong, c_ushort, c_void, wchar_t};
use std::{ptr, slice};

use crate::{
    common::traits::platform::PlatformPrinterJobGetters,
    windows::utils::{
        date::{calculate_system_time, get_current_epoch},
        memory::alloc_s,
        strings::{str_to_wide_string, wchar_t_to_string},
    },
};

#[link(name = "winspool")]
extern "system" {
    fn OpenPrinterW(
        pPrinterName: *const wchar_t,
        phPrinter: *mut *mut c_void,
        pDefault: *mut PrinterDefaultW,
    ) -> c_int;
    fn StartDocPrinterW(
        hPrinter: *mut c_void,
        Level: c_ulong,
        pDocInfo: *const DocInfo1,
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
}

#[repr(C)]
struct PrinterDefaultW {
    pDatatype: *mut wchar_t,
    pDevMode: *mut c_void,
    DesiredAccess: c_ulong,
}

#[repr(C)]
struct DocInfo1 {
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
        return self.JobId.into();
    }

    fn get_name(&self) -> String {
        return wchar_t_to_string(self.pDocument);
    }

    fn get_state(&self) -> u64 {
        return self.Status.into();
    }

    fn get_printer(&self) -> String {
        return wchar_t_to_string(self.pPrinterName);
    }

    fn get_media_type(&self) -> String {
        return wchar_t_to_string(self.pDatatype);
    }

    fn get_created_at(&self) -> std::time::SystemTime {
        return calculate_system_time(
            self.Submitted.wYear,
            self.Submitted.wMonth,
            self.Submitted.wDay,
            self.Submitted.wHour,
            self.Submitted.wMinute,
            self.Submitted.wSecond,
            self.Submitted.wMilliseconds,
        );
    }

    fn get_processed_at(&self) -> Option<std::time::SystemTime> {
        return Some(self.get_created_at());
    }

    fn get_completed_at(&self) -> Option<std::time::SystemTime> {
        return Some(self.get_created_at());
    }
}

/**
 * Print a buffer as RAW datatype with winspool WritePrinter
 */
pub fn print_buffer(
    printer_system_name: &str,
    job_name: Option<&str>,
    buffer: &[u8],
    _options: &[(&str, &str)], // currently unused
) -> Result<u64, &'static str> {
    unsafe {
        let printer_name = str_to_wide_string(printer_system_name);
        let mut printer_handle: *mut c_void = ptr::null_mut();

        if OpenPrinterW(
            printer_name.as_ptr() as *const wchar_t,
            &mut printer_handle,
            ptr::null_mut(),
        ) == 0
        {
            return Err("OpenPrinterW failed");
        }

        let mut pDocName = str_to_wide_string(job_name.unwrap_or(get_current_epoch().to_string().as_str()));
        let mut pDatatype = str_to_wide_string("RAW");

        let doc_info = DocInfo1 {
            pDocName: pDocName.as_mut_ptr() as *mut wchar_t,
            pDatatype: pDatatype.as_mut_ptr() as *mut wchar_t,
            pOutputFile: ptr::null_mut(),
        };

        let job_id = StartDocPrinterW(printer_handle, 1, &doc_info);
        if job_id == 0 {
            ClosePrinter(printer_handle);
            return Err("StartDocPrinterW failed");
        }

        if StartPagePrinter(printer_handle) == 0 {
            EndDocPrinter(printer_handle);
            ClosePrinter(printer_handle);
            return Err("StartPagePrinter failed");
        }

        let mut bytes_written: c_ulong = 0;
        let write_result = WritePrinter(
            printer_handle,
            buffer.as_ptr() as *mut c_void,
            buffer.len() as c_ulong,
            &mut bytes_written,
        );

        EndPagePrinter(printer_handle);
        EndDocPrinter(printer_handle);
        ClosePrinter(printer_handle);

        if write_result == 0 {
            return Err("WritePrinter failed")
        }

        Ok(job_id as u64)
    }
}

/**
 * Retrive print jobs of specific printer with EnumJobsW
 */
pub fn enum_printer_jobs(
    printer_system_name: &str,
) -> Result<&'static [JOB_INFO_1W], &'static str> {
    let printer_name = str_to_wide_string(printer_system_name);
    let mut printer_handle: *mut c_void = ptr::null_mut();

    if unsafe {
        OpenPrinterW(
            printer_name.as_ptr() as *const wchar_t,
            &mut printer_handle,
            ptr::null_mut(),
        )
    } == 0
    {
        return Err("OpenPrinterW failed");
    }

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
        return Err("EnumJobsW failed");
    }

    return Ok(if jobs_count > 0 {
        unsafe { slice::from_raw_parts(buffer_ptr as *const JOB_INFO_1W, jobs_count as usize) }
    } else {
        &[]
    });
}
