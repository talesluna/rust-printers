// #![allow(non_snake_case)]
//
// use crate::common::traits::platform::PlatformPrinterGetters;
// use libc::{c_int, c_uint, c_ulong, c_void, wchar_t};
// use std::io::Result;
//
// use std::{ ptr, slice };
//
// #[link(name = "winspool")]
// extern "system" {
//
//     fn EnumPrintersW(
//         Flags: c_ulong,
//         Name: *const wchar_t,
//         Level: c_uint,
//         pPrinterEnum: *mut c_void,
//         cbBuf: c_ulong,
//         pcbNeeded: *mut c_ulong,
//         pcReturned: *mut c_ulong,
//     ) -> c_int;
//
//     fn GetDefaultPrinterW(pszBuffer: *mut wchar_t, pcchBuffer: *mut c_ulong) -> c_int;
//
//     fn OpenPrinterW(pPrinterName: *const wchar_t, phPrinter: &*mut c_void, pDefault: *mut PrinterDefaultW) -> c_int;
//     fn StartDocPrinterW(hPrinter: *mut c_void, Level: c_ulong, pDocInfo: *const DocInfo1) -> c_ulong;
//     fn StartPagePrinter(hPrinter: *mut c_void) -> c_int;
//     fn WritePrinter(hPrinter: *mut c_void, pBuf: *mut c_void, cbBuf: c_ulong, pcWritten: *mut c_ulong) -> c_int;
//     fn EndPagePrinter(hPrinter: *mut c_void) -> c_int;
//     fn EndDocPrinter(hPrinter: *mut c_void) -> c_int;
//     fn ClosePrinter(hPrinter: *mut c_void) -> c_int;
// }
//
// #[derive(Debug, Clone)]
// #[repr(C)]
// pub struct PrinterInfo2w {
//     pServerName: *mut wchar_t,
//     pPrinterName: *mut wchar_t,
//     pShareName: *mut wchar_t,
//     pPortName: *mut wchar_t,
//     pDriverName: *mut wchar_t,
//     pComment: *mut wchar_t,
//     pLocation: *mut wchar_t,
//     pDevMode: *mut c_void,
//     pSepFile: *mut wchar_t,
//     pPrintProcessor: *mut wchar_t,
//     pDatatype: *mut wchar_t,
//     pParameters: *mut wchar_t,
//     pSecurityDescriptor: *mut c_void,
//     Attributes: c_ulong,
//     Priority: c_ulong,
//     DefaultPriority: c_ulong,
//     StartTime: c_ulong,
//     UntilTime: c_ulong,
//     Status: c_ulong,
//     cJobs: c_ulong,
//     AveragePPM: c_ulong,
// }
//
// #[repr(C)]
// struct DocInfo1 {
//     pDocName: *mut wchar_t,
//     pOutputFile: *mut wchar_t,
//     pDatatype: *mut wchar_t,
// }
//
// #[repr(C)]
// struct PrinterDefaultW {
//     pDatatype: *mut wchar_t,
//     pDevMode: *mut c_void,
//     DesiredAccess: c_ulong,
// }
//
//
// impl PrinterInfo2w {
//     /**
//      * Returns a string of wchar_t pointer
//      */
//     fn get_wchar_t_value(&self, s: *const wchar_t) -> String {
//         if s.is_null() {
//             return "".to_string();
//         }
//
//         let mut vec: Vec<u16> = Vec::new();
//         let mut i = 0;
//         unsafe {
//             while *s.offset(i) != 0 {
//                 vec.push(*s.offset(i) as u16);
//                 i += 1;
//             }
//         }
//         return String::from_utf16_lossy(&vec);
//     }
// }
//
// impl PlatformPrinterGetters for PrinterInfo2w {
//     /**
//      * Returns the readable name of print
//      */
//     fn get_name(&self) -> String {
//         return self.get_wchar_t_value(self.pPrinterName);
//     }
//
//     /**
//      * Returns default printer definition
//      */
//     fn get_is_default(&self) -> bool {
//         return unsafe { *self.pPrinterName == *self::get_default_printer() };
//     }
//
//     /**
//      * Returns the name of printer on system (also name)
//      */
//     fn get_system_name(&self) -> String {
//         return self.get_wchar_t_value(self.pPrinterName);
//     }
//
//     /**
//      * Returns readable name of the printer driver
//      */
//     fn get_marker_and_model(&self) -> String {
//         return self.get_wchar_t_value(self.pDriverName);
//     }
//
//     /**
//      * Return if the printer is being shared with other computers
//      */
//     fn get_is_shared(&self) -> bool {
//         return (self.Attributes & 0x00000008) == 8;
//     }
//
//     /**
//      * Return the printer uri
//      */
//     fn get_uri(&self) -> String {
//         // println!("pPrintProcessor = {:?}", self.get_wchar_t_value(self.pPrintProcessor));
//         // println!("pDatatype = {:?}", self.get_wchar_t_value(self.pDatatype));
//         // println!("pComment = {:?}", self.get_wchar_t_value(self.pComment));
//         // println!("pServerName = {:?}", self.get_wchar_t_value(self.pServerName));
//         println!("Priority = {:?}", self.Priority);
//         // println!("pParameters = {:?}", self.get_wchar_t_value(self.pParameters));
//         return "".to_string();
//     }
//
//     /**
//      * Return the printer port name
//      */
//     fn get_port_name(&self) -> String {
//         return self.get_wchar_t_value(self.pPortName);
//     }
//
//     /**
//      * Return the printer comment
//      */
//     fn get_description(&self) -> String {
//         return self.get_wchar_t_value(self.pComment);
//     }
//
//     /**
//      * Return the printer processor name
//      */
//     fn get_processor(&self) -> String {
//         return self.get_wchar_t_value(self.pPrintProcessor);
//     }
//
//     /**
//      * Return the printer data type
//      */
//     fn get_data_type(&self) -> String {
//         return self.get_wchar_t_value(self.pDatatype);
//     }
//
//     /**
//      * Return the location of the printer
//      */
//     fn get_location(&self) -> String {
//         return self.get_wchar_t_value(self.pLocation);
//     }
//
//     /**
//      * Return the state of the Winspool printer
//      */
//     fn get_state(&self) -> String {
//         return self.Status.to_string();
//     }
// }
//
// /**
//  * Returns the default system printer
//  */
// fn get_default_printer() -> *const wchar_t {
//     let mut name_size: c_ulong = 0;
//     unsafe {
//         GetDefaultPrinterW(ptr::null_mut(), &mut name_size);
//         let mut buffer: Vec<wchar_t> = vec![0; name_size as usize];
//         GetDefaultPrinterW(buffer.as_mut_ptr(), &mut name_size);
//         return buffer.as_ptr();
//     }
// }
//
// /**
//  * Returns the system printers list
//  */
// pub fn enum_printers() -> Vec<PrinterInfo2w> {
//     let mut tries = 0;
//     let mut bytes_needed: c_ulong = 0;
//     let mut count_printers: c_ulong = 0;
//
//     let mut buffer: Vec<PrinterInfo2w> = Vec::with_capacity(bytes_needed as usize);
//
//     loop {
//         if tries > 2 {
//             break;
//         }
//
//         tries += 1;
//         let buffer_ptr = buffer.as_mut_ptr();
//
//         let result = unsafe {
//             EnumPrintersW(
//                 0x00000002 | 0x00000004,
//                 ptr::null_mut(),
//                 2,
//                 buffer_ptr as *mut c_void,
//                 bytes_needed,
//                 &mut bytes_needed,
//                 &mut count_printers,
//             )
//         };
//
//         if result != 0 {
//             let sliced = unsafe { slice::from_raw_parts(buffer_ptr, count_printers as usize) };
//             for info in sliced {
//                 if !info.pPrinterName.is_null() {
//                     buffer.push(info.clone());
//                 }
//             }
//             break;
//         }
//
//         buffer.reserve(bytes_needed as usize - buffer.capacity() + 1000);
//     }
//
//     buffer.sort_by(|p,pp| pp.Priority.cmp(&p.Priority));
//
//     return buffer;
// }
//
// fn get_printer_handle(printer_name: &str) -> Result<*mut c_void> {
//     let printer_name_wide = wide_string(printer_name);
//     let mut printer_handle: *mut c_void = ptr::null_mut();
//
//     unsafe {
//         if OpenPrinterW(printer_name_wide, &mut printer_handle, ptr::null_mut()) == 0 {
//             return Err(std::io::Error::last_os_error());
//         }
//     }
//
//     Ok(printer_handle)
// }
//
// pub fn print_to_printer(printer_name: &str, document_name: &str, job_name: &str, document_content: &[u8]) -> Result<()> {
//     let printer_handle = get_printer_handle(printer_name)?;
//
//     let doc_info = DocInfo1 {
//         pDocName: wide_string(document_name),
//         pOutputFile: ptr::null_mut(),
//         pDatatype: wide_string("XPS").to_owned()
//     };
//
//     unsafe {
//         let job_id = StartDocPrinterW(printer_handle, 1, &doc_info);
//         if job_id == 0 {
//             return Err(std::io::Error::last_os_error());
//         }
//
//         StartPagePrinter(printer_handle);
//
//         let mut bytes_written: c_ulong = 0;
//         WritePrinter(printer_handle, document_content.as_ptr() as *mut c_void, document_content.len() as c_ulong, &mut bytes_written);
//
//         EndPagePrinter(printer_handle);
//         EndDocPrinter(printer_handle);
//         ClosePrinter(printer_handle);
//     }
//
//     Ok(())
// }
//
// fn wide_string(value: &str) -> *mut wchar_t{
//     let mut wide: Vec<u16> = value.encode_utf16().chain(Some(0)).collect();
//     return wide.as_mut_ptr() as *mut wchar_t;
// }