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
    document_name: &str,
    job_name: &str,
    document_content: &[u8],
) -> bool {
    let printer_name = utils::strings::str_to_wchar_t_ptr(printer_name);

    let mut printer_handle: *mut c_void = ptr::null_mut();

    unsafe {
        println!("A0 {:?}", printer_handle);

        let open_result = OpenPrinterW(printer_name, &mut printer_handle, ptr::null_mut());

        println!("A1 {:?}", printer_handle);
        if open_result == 0 {
            return false;
        }

        let doc_info = DocInfo1 {
            pDocName: utils::strings::str_to_wchar_t_ptr(document_name),
            pOutputFile: ptr::null_mut(),
            pDatatype: utils::strings::str_to_wchar_t_ptr("RAW").to_owned(),
        };

        let start_result = StartDocPrinterW(printer_handle, 1, &doc_info);
        println!("A2 {:?}", start_result);

        if start_result == 0 {
            ClosePrinter(printer_handle);
            return false;
        }

        let start_pp_result = StartPagePrinter(printer_handle);
        println!("A3 {:?}", start_pp_result);

        if start_pp_result == 0 {
            EndDocPrinter(printer_handle);
            ClosePrinter(printer_handle);
            return false;
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

        println!("A3 {:?} {:?}", bytes_written, write_result);

    }

    return true;
}
