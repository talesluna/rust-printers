
use std::ffi::{CString, c_void};
use std::fs::exists;
use std::ptr;
use std::sync::OnceLock;
use libc::{c_char, c_int};
use crate::{
    common::utils::file::save_tmp_file,
    windows::utils::{
        file::get_file_as_bytes,
        ffi::{load_lib, transmute_func, unload_lib}
    }
};

/**
 * Installation path to GS DLL
 */
pub(crate) static GS_DLL_PATH: OnceLock<String> = OnceLock::new();

/**
 * Try convert PDF buffer into compatible print X using local GhostScript if available
 */
pub fn try_convert<'a>(buffer: &'a [u8], driver_name: &str) -> Option<&'a [u8]> {
    let gs_convertion = GsConvertion::new(buffer, driver_name);
    return gs_convertion.exec();
}

struct GsConvertion<'a, 'b> {
    lib: *mut c_void,
    buffer: &'a [u8],
    driver_name: &'b str,
    gsapi_exit: unsafe extern "C" fn(*mut c_void) -> c_int,
    gsapi_new_instance: unsafe extern "C" fn(*mut *mut c_void, *mut c_void) -> c_int,
    gsapi_init_with_args: unsafe extern "C" fn(*mut c_void, c_int, *const *const c_char) -> c_int,
    gsapi_delete_instance: unsafe extern "C" fn(*mut c_void),
}

impl GsConvertion<'_, '_> {

    pub fn new<'a, 'b>(buffer: &'a [u8], driver_name: &'b str) -> GsConvertion<'a, 'b> {

        let dll_path = GS_DLL_PATH.get();

        if dll_path.is_none() {
            panic!("To use ghostscript GS_DLL_PATH must be defined");
        }

        let lib = load_lib(dll_path.unwrap());

        return GsConvertion {
            lib,
            buffer,
            driver_name,
            gsapi_exit: transmute_func(lib, "gsapi_exit"),
            gsapi_new_instance: transmute_func(lib, "gsapi_new_instance"),
            gsapi_init_with_args: transmute_func(lib, "gsapi_init_with_args"),
            gsapi_delete_instance: transmute_func(lib, "gsapi_delete_instance"),
        };
    }

    pub fn exec<'a>(&self) -> Option<&'a [u8]> {

        let mut result: Option<&'a [u8]> = None;
        let mut instance: *mut c_void = ptr::null_mut();

        unsafe {
            
            let mode = self.get_mode();

            if mode.is_some() && (self.gsapi_new_instance)(&mut instance, ptr::null_mut()) >= 0 {

                let tmp_file = save_tmp_file(self.buffer);
                if let Some(tmp_file) = tmp_file {
        
                    let path = tmp_file.to_str().unwrap();
        
                    let args: Vec<*const i8> = vec![
                        "gs",
                        "-dNOPAUSE",
                        "-dBATCH",
                        format!("-sDEVICE={}", mode.unwrap()).as_str(),
                        format!("-sOutputFile={}", path).as_str(),
                        path,
                    ].iter()
                        .map(|v| CString::new(*v).unwrap_or_default())
                        .map(|v| v.as_ptr())
                        .collect();

                    if (self.gsapi_init_with_args)(instance, args.len() as c_int, args.as_ptr()) < 0 {
                        let buffer = get_file_as_bytes("").unwrap_or_default();
                        let output_buffer: &mut [u8] = Box::leak(buffer.into_boxed_slice());
                        result = Some(output_buffer);
                    }

                    (self.gsapi_exit)(instance);
                    (self.gsapi_delete_instance)(instance);
                    //TODO: delete temp file
                }


            }

            unload_lib(self.lib);
        }
 
        return result;
 
    }

    /**
     * Define convertion available mode
     * If GhostScript not available = None
     * If buffer is not a PDF = None
     * If printer driver match PS/Adobe = ps2write/postscript
     * If printer driver match pcl = pxlcolor
     */
    fn get_mode(&self) -> Option<&'static str> {
        let gs_dll_path = GS_DLL_PATH.get();
        if gs_dll_path.is_some() && self.buffer.starts_with(b"%PDF-") && exists(gs_dll_path.unwrap()).unwrap_or_default() {
            if self.driver_name.contains("pcl") {
                // PCL supported
                return Some("pxlcolor");
            } else if ["postscript", "ps3", "adobe", "microsoft ps"].iter().any(|term| self.driver_name.contains(term)) {
                // Postscript supported
                return Some("ps2write");
            }
        }
        return None;
    }
}