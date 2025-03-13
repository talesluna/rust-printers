#![allow(non_snake_case)]

use libc::{c_int, c_uint, c_ulong, c_void, wchar_t};
use std::{ptr, slice};

use crate::{
    common::traits::platform::PlatformPrinterGetters,
    windows::utils::{memory::{alloc_s, dealloc_s}, strings::{str_to_wide_string, wchar_t_to_string}}
};

#[link(name = "winspool")]
extern "system" {

    fn EnumPrintersW(
        Flags: c_ulong,
        Name: *const wchar_t,
        Level: c_uint,
        pPrinterEnum: *mut PRINTER_INFO_2W,
        cbBuf: c_ulong,
        pcbNeeded: *mut c_ulong,
        pcReturned: *mut c_ulong,
    ) -> c_int;

    fn GetDefaultPrinterW(pszBuffer: *mut wchar_t, pcchBuffer: *mut c_ulong) -> c_int;

}

/**
 * The winspool PRINTER_INFO_2 structure specifies detailed printer information.
 * https://learn.microsoft.com/en/windows/win32/printdocs/printer-info-2
 */
#[derive(Debug, Clone)]
#[repr(C)]
pub struct PRINTER_INFO_2W {
    pServerName: *mut wchar_t,
    pub pPrinterName: *mut wchar_t,
    pShareName: *mut wchar_t,
    pPortName: *mut wchar_t,
    pDriverName: *mut wchar_t,
    pComment: *mut wchar_t,
    pLocation: *mut wchar_t,
    pDevMode: *mut c_void,
    pSepFile: *mut wchar_t,
    pPrintProcessor: *mut wchar_t,
    pDatatype: *mut wchar_t,
    pParameters: *mut wchar_t,
    pSecurityDescriptor: *mut c_void,
    Attributes: c_ulong,
    Priority: c_ulong,
    DefaultPriority: c_ulong,
    StartTime: c_ulong,
    UntilTime: c_ulong,
    Status: c_ulong,
    cJobs: c_ulong,
    AveragePPM: c_ulong,
}

impl PlatformPrinterGetters for PRINTER_INFO_2W {
    fn get_name(&self) -> String {
        return wchar_t_to_string(self.pPrinterName);
    }
    fn get_is_default(&self) -> bool {
        let mut name_size: c_ulong = 0;
        return unsafe {
            GetDefaultPrinterW(ptr::null_mut(), &mut name_size);
            let mut buffer: Vec<wchar_t> = vec![0; name_size as usize];
            GetDefaultPrinterW(buffer.as_mut_ptr(), &mut name_size);
            *self.pPrinterName == *buffer.as_ptr()
        };
    }
    fn get_system_name(&self) -> String {
        return wchar_t_to_string(self.pPrinterName);
    }
    fn get_marker_and_model(&self) -> String {
        return wchar_t_to_string(self.pDriverName);
    }
    fn get_is_shared(&self) -> bool {
        return (self.Attributes & 0x00000008) == 8;
    }
    fn get_uri(&self) -> String {
        return "".to_string();
    }
    fn get_location(&self) -> String {
        return wchar_t_to_string(self.pLocation);
    }
    fn get_state(&self) -> String {
        return self.Status.to_string();
    }
    fn get_state_reasons(&self) -> Vec<String> {
        // TODO: Implement
        return vec![];
    }
    fn get_port_name(&self) -> String {
        return wchar_t_to_string(self.pPortName);
    }
    fn get_processor(&self) -> String {
        return wchar_t_to_string(self.pPrintProcessor);
    }
    fn get_description(&self) -> String {
        return wchar_t_to_string(self.pComment);
    }
    fn get_data_type(&self) -> String {
        return wchar_t_to_string(self.pDatatype);
    }
}

/**
 * Returns all available printer using EnumPrintersW
 */
pub fn enum_printers(name: Option<&str>) -> &'static [PRINTER_INFO_2W] {
    let mut bytes_needed: c_ulong = 0;
    let mut count_printers: c_ulong = 0;
    let mut buffer_ptr: *mut PRINTER_INFO_2W = ptr::null_mut();
    let name_ptr = if name.is_none() {
        ptr::null_mut()
    } else {
        str_to_wide_string(name.unwrap()).as_ptr()
    } as *const wchar_t;

    for _ in 0..2 {
        let result = unsafe {
            EnumPrintersW(
                0x00000002 | 0x00000004,
                name_ptr,
                2,
                buffer_ptr,
                bytes_needed,
                &mut bytes_needed,
                &mut count_printers,
            )
        };

        if result != 0 || bytes_needed == 0 {
            break;
        }

        buffer_ptr = alloc_s::<PRINTER_INFO_2W>(bytes_needed);

    }

    return unsafe { slice::from_raw_parts(buffer_ptr, count_printers as usize) };
}

/**
 * Returns the defualt printer filetring all printer
 */
pub fn get_default_printer() -> Option<&'static PRINTER_INFO_2W> {
    return enum_printers(None).iter().find(|p| p.get_is_default());
}

/**
 * Free winspool printer memory
 */
pub fn free(printers: &'static [PRINTER_INFO_2W]) {
    if printers.len() > 0 {
        dealloc_s::<PRINTER_INFO_2W>(printers.as_ptr());
    }
}
