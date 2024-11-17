#![allow(non_snake_case)]

use libc::{c_int, c_uint, c_ulong, c_void, wchar_t};
use std::{
    alloc::Layout,
    alloc::{alloc, dealloc},
    mem::{align_of, size_of},
    ptr,
    slice
};

use crate::common::{traits::platform::PlatformPrinterGetters, utils};

#[link(name = "winspool")]
extern "system" {

    fn EnumPrintersW(
        Flags: c_ulong,
        Name: *const wchar_t,
        Level: c_uint,
        pPrinterEnum: *mut PrinterInfo2w,
        cbBuf: c_ulong,
        pcbNeeded: *mut c_ulong,
        pcReturned: *mut c_ulong,
    ) -> c_int;

    fn GetDefaultPrinterW(pszBuffer: *mut wchar_t, pcchBuffer: *mut c_ulong) -> c_int;

}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct PrinterInfo2w {
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

impl PlatformPrinterGetters for PrinterInfo2w {
    /**
     * Returns the readable name of print
     */
    fn get_name(&self) -> String {
        return utils::strings::wchar_t_to_string(self.pPrinterName);
    }

    /**
     * Returns default printer definition
     */
    fn get_is_default(&self) -> bool {
        let mut name_size: c_ulong = 0;
        return unsafe {
            GetDefaultPrinterW(ptr::null_mut(), &mut name_size);
            let mut buffer: Vec<wchar_t> = vec![0; name_size as usize];
            GetDefaultPrinterW(buffer.as_mut_ptr(), &mut name_size);
            *self.pPrinterName == *buffer.as_ptr()
        };
    }

    /**
     * Returns the name of printer on system (also name)
     */
    fn get_system_name(&self) -> String {
        return utils::strings::wchar_t_to_string(self.pPrinterName);
    }

    /**
     * Returns readable name of the printer driver
     */
    fn get_marker_and_model(&self) -> String {
        return utils::strings::wchar_t_to_string(self.pDriverName);
    }

    /**
     * Return if the printer is being shared with other computers
     */
    fn get_is_shared(&self) -> bool {
        return (self.Attributes & 0x00000008) == 8;
    }

    /**
     * Return the printer uri
     */
    fn get_uri(&self) -> String {
        return "".to_string();
    }

    /**
     * Return the location of the printer
     */
    fn get_location(&self) -> String {
        return utils::strings::wchar_t_to_string(self.pLocation);
    }

    /**
     * Return the state of the Winspool printer
     */
    fn get_state(&self) -> String {
        return self.Status.to_string();
    }

    /**
     * Return the printer port name
     */
    fn get_port_name(&self) -> String {
        return utils::strings::wchar_t_to_string(self.pPortName);
    }

    /**
     * Return the printer processor name
     */
    fn get_processor(&self) -> String {
        return utils::strings::wchar_t_to_string(self.pPrintProcessor);
    }

    /**
     * Return the printer comment
     */
    fn get_description(&self) -> String {
        return utils::strings::wchar_t_to_string(self.pComment);
    }

    /**
     * Return the printer data type
     */
    fn get_data_type(&self) -> String {
        return utils::strings::wchar_t_to_string(self.pDatatype);
    }
}

fn ptr_layout(size: usize) -> Layout {
    return unsafe { Layout::from_size_align_unchecked(size, align_of::<PrinterInfo2w>()) };
}

/**
 * Returns the system printers list
 */
pub fn enum_printers(name: Option<&str>) -> &'static [PrinterInfo2w] {
    let mut bytes_needed: c_ulong = 0;
    let mut count_printers: c_ulong = 0;
    let mut buffer_ptr: *mut PrinterInfo2w = ptr::null_mut();
    let name_ptr = if name.is_none() { ptr::null_mut() } else { utils::strings::str_to_wchar_t_ptr(name.unwrap()) } as *const wchar_t;

    // println!(">> name_ptr={:?}", utils::strings::wchar_t_to_string(name_ptr));

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

        if result == 0 {
            unsafe {
                buffer_ptr = alloc(ptr_layout(bytes_needed as usize)) as *mut PrinterInfo2w;
            }
        }
    }

    return unsafe { slice::from_raw_parts(buffer_ptr, count_printers as usize) };
}

pub fn get_default_printer() -> Option<&'static PrinterInfo2w> {
    return enum_printers(None).iter().find(|p| p.get_is_default());
}

pub fn free(printers: &'static [PrinterInfo2w]) {
    if printers.len() > 0 {
        unsafe {
            let ptr: *const PrinterInfo2w = printers.as_ptr();
            let layout = ptr_layout(size_of::<PrinterInfo2w>());
            dealloc(ptr as *mut u8, layout);
        }
    }
}
