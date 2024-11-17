extern crate libc;
extern crate printers;

use std::{convert::TryInto, ffi::CStr, iter::FromIterator};
use libc::{ c_uchar, c_int, c_void};

#[repr(C)]
pub struct PrinterC {
    pub name: [c_uchar; 64],
    pub system_name: [c_uchar; 64],
    pub driver_name: [c_uchar; 64],
    pub uri: [c_uchar; 64],
    pub location: [c_uchar; 64],
    pub is_default: bool,
    pub is_shared: bool,
    pub state: c_int,
}

#[repr(C)]
pub struct PrintBufferC {
    pub size: c_int,
    pub buffer: *const c_uchar,
    pub job_name: *const c_uchar,
    pub printer_name: *const c_uchar,
}

#[repr(C)]
pub struct PrintFileC {
    pub job_name: *const c_uchar,
    pub file_path: *const c_uchar,
    pub printer_name: *const c_uchar,
}

#[repr(C)]
pub struct GetPrintersResult {    
    pub len: i32,
    pub item_size: i32,
    pub printers: *const PrinterC
}

fn c_uchar_ptr_to_str(value: *const c_uchar) -> &'static str {
    let c_str = unsafe {
        CStr::from_ptr(value as *const i8)
    };
    return c_str.to_str().unwrap();
}

fn string_to_array(value: String) -> [c_uchar; 64] {
    let mut array: [c_uchar; 64] = [0; 64];
    value.as_bytes().iter().enumerate().for_each(|(i, &byte)| {
        array[i] = byte as c_uchar;
    });

    return array;
}

fn get_job_name(value: *const c_uchar) -> Option<&'static str> {
    return unsafe { value.as_ref() }.map(|value| c_uchar_ptr_to_str(value));
}

fn printer_to_printer_c(printer: &printers::printer::Printer) -> PrinterC {
    return PrinterC {
        uri: string_to_array(printer.uri.to_owned()),
        name: string_to_array(printer.name.to_owned()),
        system_name: string_to_array(printer.system_name.to_owned()),
        driver_name: string_to_array(printer.driver_name.to_owned()),
        location: string_to_array(printer.location.to_owned()),
        is_default: printer.is_default.to_owned(),
        is_shared: printer.is_shared.to_owned(),
        state: printer.state.to_owned() as c_int,
    }
}

#[no_mangle]
pub extern fn print(args: PrintBufferC) -> c_int {
    let job_name = get_job_name(args.job_name);
    let printer_name = c_uchar_ptr_to_str(args.printer_name);

    let buffer = unsafe {
        std::slice::from_raw_parts(args.buffer, args.size as usize)
    };

    let result = printers::print(printer_name, buffer, job_name);
    return if result.is_ok() { 1 } else { 0 };
}


#[no_mangle]
pub extern fn print_file(args: PrintFileC) -> c_int {
    let file_path = c_uchar_ptr_to_str(args.file_path);
    let job_name = get_job_name(args.job_name);
    let printer_name = c_uchar_ptr_to_str(args.printer_name);

    let result = printers::print_file(printer_name, file_path, job_name);

    return if result.is_ok() { 1 } else { 0 };
}

#[no_mangle]
pub extern fn get_printers() -> GetPrintersResult {
    let all =  printers::get_printers();

    let list = all.iter().map(|p| printer_to_printer_c(p));

    let list_box = Box::from_iter(list);

    return GetPrintersResult {
        len: all.len().try_into().unwrap(),
        item_size: std::mem::size_of::<PrinterC>().try_into().unwrap(),
        printers: Box::into_raw(list_box) as *const PrinterC,
    }
}

#[no_mangle]
pub extern fn get_printer_by_name(name: *const c_uchar) -> *const PrinterC {
    let name = c_uchar_ptr_to_str(name);
    let list = printers::get_printers();

    let o_printer = list.iter().find(|&printer| {
        return printer.clone().name.eq(name) || printer.clone().system_name.eq(name);
    });

    return if o_printer.is_none() {
        std::ptr::null()
    } else {
        let printer = o_printer.unwrap();
        let printer_c = printer_to_printer_c(printer);
        return Box::into_raw(Box::new(printer_c));
    }

}

#[no_mangle]
pub extern fn free(ptr: *const PrinterC) -> *const c_void {
    if !ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(ptr as *mut PrinterC);
        }
    }

    return std::ptr::null();
}