// use libc::{c_char, c_int, time_t};
// use std::{
//     ffi::CString,
//     iter::FromIterator,
//     ptr,
//     slice,
// };

// use crate::common::{
//     traits::platform::{PlatformPrinterGetters, PlatformPrinterJobGetters},
//     utils::strings::c_char_to_str,
// };

// /**
//  * The CUPS option struct (cups_option_s)
//  * https://www.cups.org/doc/cupspm.html#cups_option_s
//  */
// #[derive(Debug, Clone, Copy)]
// #[repr(C)]
// pub struct CupsOptionT {
//     pub name: *mut c_char,
//     pub value: *mut c_char,
// }

// /**
//  * The CUPS destination struct (cups_dest_s)
//  * https://www.cups.org/doc/cupspm.html#cups_dest_s
//  */
// #[derive(Debug, Clone, Copy)]
// #[repr(C)]
// pub struct CupsDestT {
//     name: *mut c_char,
//     instance: *mut c_char,
//     is_default: c_int,
//     num_options: c_int,
//     options: *mut CupsOptionT,
// }

// #[derive(Debug, Clone, Copy)]
// #[repr(C)]
// pub struct CupsJobsS {
//     pub id: c_int,
//     pub user: *mut c_char,
//     pub title: *mut c_char,
//     pub format: *mut c_char,
//     pub priority: c_int,
//     pub copies: c_int,
//     pub state: c_int,
//     pub state_message: *mut c_char,
//     pub dest: *mut c_char,
//     pub created: time_t,
//     pub completed: time_t,
//     pub processing: time_t,
//     pub progress: c_int,
//     pub job_k_octets: c_int,
//     pub job_impressions: c_int,
//     pub media: *mut c_char,
//     pub printer: *mut c_char,
// }


// #[link(name = "cups")]
// extern "C" {
//     fn cupsGetDests(dests: *mut *mut CupsDestT) -> c_int;

//     fn cupsFreeDests(num_dests: c_int, dests: *const CupsDestT);

//     fn cupsPrintFile(
//         printer_name: *const c_char,
//         filename: *const c_char,
//         title: *const c_char,
//         options: i32,
//     ) -> i32;

//     fn cupsGetOption(
//         name: *const c_char,
//         num_options: c_int,
//         options: *mut CupsOptionT,
//     ) -> *const c_char;

//     fn cupsGetJobs(
//         jobs: *mut *mut CupsJobsS,
//         name: *const c_char,
//         how: c_int,
//         from: c_int,
//         to: c_int,
//     ) -> c_int;

// }

// impl CupsDestT {
//     fn get_option(&self, key: &str) -> &'static str {
//         let key = CString::new(key);
//         let mut value = "";

//         if !self.options.is_null() && key.is_ok() {
//             let option_key = key.unwrap();
//             unsafe {
//                 let option_value = cupsGetOption(option_key.as_ptr(), self.num_options, self.options);
//                 if !option_value.is_null() {
//                     value = c_char_to_str(option_value);
//                 }
//             };
//         }

//         return value;
//     }

//     pub fn is_shared_duplex(&self) -> bool {
//         return self.num_options == 5;
//     }
// }

// impl PlatformPrinterGetters for CupsDestT {
//     /**
//      * Returns the name of the destination
//      */
//     fn get_system_name(&self) -> &'static str {
//         return c_char_to_str(self.name);
//     }

//     /**
//      * Returns default destination definition
//      */
//     fn get_is_default(&self) -> bool {
//         return self.is_default == 1;
//     }

//     /**
//      * Returns readable name of dest by "printer-info" option
//      */
//     fn get_name(&self) -> &'static str {
//         return self.get_option("printer-info");
//     }

//     /**
//      * Returns readable name of the dest driver by "printer-make-and-model" option
//      */
//     fn get_marker_and_model(&self) -> &'static str {
//         return self.get_option("printer-make-and-model");
//     }

//     /**
//      * Return if the destination is being shared with other computers
//      */
//     fn get_is_shared(&self) -> bool {
//         return self.get_option("printer-is-shared") == "true";
//     }

//     /**
//      * Return the printer URI
//      */
//     fn get_uri(&self) -> &'static str {
//         return self.get_option("printer-uri-supported");
//     }
    
//     /**
//      * Return the printer port name
//      */
//     fn get_port_name(&self) -> &'static str {
//         return self.get_option("device-uri");
//     }

//     /**
//      * Return the printer comment
//      */
//     fn get_description(&self) -> &'static str {
//         return "";
//     }

//     /**
//      * Return the printer processor name
//      */
//     fn get_processor(&self) -> &'static str {
//         return "";
//     }

//     /**
//      * Return the printer data type
//      */
//     fn get_data_type(&self) -> &'static str {
//         return self.get_option("media");
//     }

//     /**
//      * Return the location option
//      */
//     fn get_location(&self) -> &'static str {
//         return self.get_option("printer-location");
//     }

//     /**
//      * Return the state of the CUPS printer
//      */
//     fn get_state(&self) -> &'static str {
//         return self.get_option("printer-state");
//     }
// }

// impl PlatformPrinterJobGetters for CupsJobsS {

//     fn get_id(&self) -> &'static str {
//         return "";
//     }

//     fn get_name(&self) -> &'static str {
//         return c_char_to_str(self.title);
//     }

//     fn get_state(&self) -> &'static str {
//         return "";
//     }

//     fn get_printer(&self) -> &'static str {
//         return c_char_to_str(self.printer);
//     }
// }

// /**
//  * Returns a vector of CupsDestT (cups_dest_s) struct with all available destinations
//  * Using cupsGetDests
//  */
// pub fn get_dests() -> &'static [CupsDestT] {
//     unsafe {
//         let mut dests_ptr: *mut CupsDestT = ptr::null_mut();
//         let dests_count: i32 = cupsGetDests(&mut dests_ptr);
//         return slice::from_raw_parts(dests_ptr, dests_count as usize);
//     }
// }

// /**
//  * 
//  */
// pub fn get_printer_jobs(printer_name: &str, active_only: bool) -> &'static [CupsJobsS] {
//     let mut jobs_ptr: *mut CupsJobsS = std::ptr::null_mut();

//     let how = if active_only { 2 } else { 1 };
//     let name = printer_name.as_ptr() as *const c_char;

//     return unsafe { 
//         let jobs_count = cupsGetJobs(&mut jobs_ptr, name, how, 0, 0);
//         slice::from_raw_parts(jobs_ptr, jobs_count as usize)
//     };

// }

// /**
//  * Send an file to printer
//  */
// pub fn print_file(printer_name: &str, file_path: &str, job_name: Option<&str>) -> bool {
//     unsafe {
//         let printer_name = CString::new(printer_name).unwrap();
//         let filename = CString::new(file_path).unwrap();
//         let title = CString::new(job_name.unwrap_or(file_path)).unwrap();

//         let result = cupsPrintFile(printer_name.as_ptr(), filename.as_ptr(), title.as_ptr(), 0);
//         return result != 0;
//     }
// }

// /**
//  *
//  */
// pub fn free(dests: &'static [CupsDestT]) {
//     if dests.len() > 0 {
//         unsafe {
//             let ptr = Box::from_iter(dests).as_ptr();
//             cupsFreeDests(dests.len() as i32, *ptr);
//         }
//     }
// }
