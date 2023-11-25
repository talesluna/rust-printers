#![allow(non_snake_case)]

use crate::shared::interface::PlatformPrinterGetters;
use libc::{c_int, c_uint, c_ulong, c_void, wchar_t};
use std::{ ptr, slice };

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

    fn GetDefaultPrinterW(pszBuffer: *mut wchar_t, pcchBuffer: *mut c_ulong) -> c_int;

}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct PrinterDefaults {
    pDatatype: *mut wchar_t,
    pDevMode: *mut c_void,
    DesiredAccess: c_ulong,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct AddJobInfo2 {
    job_level: c_ulong,
    printer_name: *const wchar_t,
    document_name: *const wchar_t,
    datatype: *const wchar_t,
    status: *mut wchar_t,
}

impl PrinterInfo2w {
    /**
     * Returns a string of wchar_t pointer
     */
    fn get_wchar_t_value(&self, s: *const wchar_t) -> String {
        if s.is_null() {
            return "".to_string();
        }

        let mut vec: Vec<u16> = Vec::new();
        let mut i = 0;
        unsafe {
            while *s.offset(i) != 0 {
                vec.push(*s.offset(i) as u16);
                i += 1;
            }
        }
        return String::from_utf16_lossy(&vec);
    }
}

impl PlatformPrinterGetters for PrinterInfo2w {
    /**
     * Returns the readable name of print
     */
    fn get_name(&self) -> String {
        return self.get_wchar_t_value(self.pPrinterName);
    }

    /**
     * Returns default printer definition
     */
    fn get_is_default(&self) -> bool {
        return unsafe { *self.pPrinterName == *self::get_default_printer() };
    }

    /**
     * Returns the name of printer on system (also name)
     */
    fn get_system_name(&self) -> String {
        return self.get_wchar_t_value(self.pPrinterName);
    }

    /**
     * Returns readable name of the printer driver
     */
    fn get_marker_and_model(&self) -> String {
        return self.get_wchar_t_value(self.pDriverName);
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
        return self.get_wchar_t_value(self.pLocation);
    }

    /**
     * Return the state of the Winspool printer
     */
    fn get_state(&self) -> String {
        return self.Status.to_string();
    }
}

/**
 * Returns the default system printer
 */
fn get_default_printer() -> *const wchar_t {
    let mut name_size: c_ulong = 0;
    unsafe {
        GetDefaultPrinterW(ptr::null_mut(), &mut name_size);
        let mut buffer: Vec<wchar_t> = vec![0; name_size as usize];
        GetDefaultPrinterW(buffer.as_mut_ptr(), &mut name_size);
        return buffer.as_ptr();
    }
}

/**
 * Returns the system printers list
 */
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
                0x00000002 | 0x00000004,
                ptr::null_mut(),
                2,
                buffer_ptr as *mut c_void,
                bytes_needed,
                &mut bytes_needed,
                &mut count_printers,
            )
        };

        if result != 0 {
            let sliced = unsafe { slice::from_raw_parts(buffer_ptr, count_printers as usize) };
            for info in sliced {
                if !info.pPrinterName.is_null() {
                    buffer.push(info.clone());
                }
            }
            break;
        }

        buffer.reserve(bytes_needed as usize - buffer.capacity() + 1000);
    }

    return buffer;
}
