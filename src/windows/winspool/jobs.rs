#![allow(non_snake_case)]

use libc::{c_int, c_ulong, c_void, wchar_t};
use std::ptr;

use crate::common::utils;

#[link(name = "winspool")]
extern "system" {
    fn OpenPrinterW(
        pPrinterName: *const wchar_t,
        phPrinter: &*mut c_void,
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

pub fn print_to_printer(
    printer_name: &str,
    job_name: &str,
    document_content: &[u8],
) -> Result<(), &'static str> {    
    unsafe {

        let printer_name = utils::strings::str_to_wide_string(printer_name);
        let mut printer_handle: *mut c_void = ptr::null_mut();

        if OpenPrinterW(printer_name.as_ptr() as *const wchar_t, &mut printer_handle, ptr::null_mut()) == 0 {
            return Err("OpenPrinterW failed");
        }

        let mut pDocName = utils::strings::str_to_wide_string(job_name);
        let mut pDatatype = utils::strings::str_to_wide_string("RAW");

        let doc_info = DocInfo1 {
            pDocName: pDocName.as_mut_ptr() as *mut wchar_t,
            pDatatype: pDatatype.as_mut_ptr() as *mut wchar_t,
            pOutputFile: ptr::null_mut(),
        };
        
        if StartDocPrinterW(printer_handle, 1, &doc_info) == 0 {
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
            document_content.as_ptr() as *mut c_void,
            document_content.len() as c_ulong,
            &mut bytes_written,
        );

        EndPagePrinter(printer_handle);
        EndDocPrinter(printer_handle);
        ClosePrinter(printer_handle);

        if write_result == 0 {
            return Err("WritePrinter failed")
        }
    }

    return Ok(());

}
