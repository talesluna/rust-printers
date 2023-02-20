#![allow(non_snake_case)]

use libc::{wchar_t, c_void, c_ulong, c_int, c_uint};
use std::{ptr, slice};

#[link(name = "winspool")]
extern "system" {

    fn EnumPrintersW(
        Flags: c_ulong,
        Name: *const wchar_t,
        Level: c_uint,
        pPrinterEnum: *mut c_void,
        cbBuf: c_ulong,
        pcbNeeded: *mut c_ulong,
        pcReturned: *mut c_ulong,
    ) -> c_int;
}


#[derive(Debug, Clone)]
#[repr(C)]
pub struct PrinterInfo2w {
    pub pServerName: *mut wchar_t,
    pub pPrinterName: *mut wchar_t,
    pub pShareName: *mut wchar_t,
    pub pPortName: *mut wchar_t,
    pub pDriverName: *mut wchar_t,
    pub pComment: *mut wchar_t,
    pub pLocation: *mut wchar_t,
    pub pDevMode: *mut c_void,
    pub pSepFile: *mut wchar_t,
    pub pPrintProcessor: *mut wchar_t,
    pub pDatatype: *mut wchar_t,
    pub pParameters: *mut wchar_t,
    pub pSecurityDescriptor: *mut c_void,
    pub Attributes: c_ulong,
    pub Priority: c_ulong,
    pub DefaultPriority: c_ulong,
    pub StartTime: c_ulong,
    pub UntilTime: c_ulong,
    pub Status: c_ulong,
    pub cJobs: c_ulong,
    pub AveragePPM: c_ulong,
}

pub fn enum_printers() -> Vec<PrinterInfo2w> {

    let mut tries = 0;
    let mut bytes_needed: c_ulong = 0;
    let mut count_printers: c_ulong = 0;
    
    let mut buffer: Vec<PrinterInfo2w> = Vec::with_capacity(bytes_needed as usize);
    
    loop {

        if tries > 2 {
            break;
        }

        tries += 1;
        let buffer_ptr = buffer.as_mut_ptr();

        let result = unsafe {
            EnumPrintersW(
                2 | 4,
                ptr::null_mut(),
                2,
                buffer_ptr as *mut c_void,
                bytes_needed,
                &mut bytes_needed,
                &mut count_printers,
            )
        };

        if result != 0 {

            let sliced = unsafe { slice::from_raw_parts(
                buffer_ptr, 
                count_printers as usize
            )};

            for info in sliced {
                if !info.pPortName.is_null() {
                    buffer.push(info.clone());
                }
            }

            break;
        }

        buffer.reserve(bytes_needed as usize - buffer.capacity() + 1000);
    }

    return buffer;

}
