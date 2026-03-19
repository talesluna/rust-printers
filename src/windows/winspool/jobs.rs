#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use libc::{c_int, c_short, c_ulong, c_ushort, c_void, wchar_t};
use std::{
    alloc::{Layout, alloc, dealloc},
    ffi::c_char,
    ptr, slice,
};

use crate::{
    common::{
        base::{
            devmode::{parse_color_model, parse_duplex, parse_media_source, parse_paper_size},
            errors::PrintersError,
        },
        traits::platform::PlatformPrinterJobGetters,
    },
    windows::utils::{
        date::{calculate_system_time, get_current_epoch},
        memory::alloc_s,
        strings::{str_to_wide_string, wchar_t_to_string},
    },
};

#[link(name = "winspool")]
unsafe extern "system" {
    fn OpenPrinterW(
        pPrinterName: *const wchar_t,
        phPrinter: *mut *mut c_void,
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
    fn EnumJobsW(
        hPrinter: *mut c_void,
        firstJob: c_ulong,
        noJobs: c_ulong,
        level: c_ulong,
        pJob: *mut c_void,
        cbBuf: c_ulong,
        pcbNeeded: *mut c_ulong,
        pcReturned: *mut c_ulong,
    ) -> c_int;
    fn SetJobW(
        hPrinter: *mut c_void,
        JobId: c_ulong,
        Level: c_ulong,
        pJob: *mut c_char,
        Command: c_ulong,
    ) -> c_int;
    fn DocumentPropertiesW(
        hWnd: *mut c_void,
        hPrinter: *mut c_void,
        pDeviceName: *const wchar_t,
        pDevModeOutput: *mut c_void,
        pDevModeInput: *mut c_void,
        fMode: c_ulong,
    ) -> c_int;
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

// DEVMODE field flags
const DM_PAPERSIZE: c_ulong = 0x00000002;
const DM_PAPERLENGTH: c_ulong = 0x00000004;
const DM_PAPERWIDTH: c_ulong = 0x00000008;
const DM_COPIES: c_ulong = 0x00000100;
const DM_DEFAULTSOURCE: c_ulong = 0x00000200;
const DM_COLOR: c_ulong = 0x00000800;
const DM_DUPLEX: c_ulong = 0x00001000;

// DocumentPropertiesW fMode
const DM_OUT_BUFFER: c_ulong = 2;

// OpenPrinterW access rights
const PRINTER_ACCESS_USE: c_ulong = 0x00000008;

/**
 * The DEVMODEW structure contains information about the initialization and environment of a printer.
 * https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-devmodew
 *
 * Only the printer-relevant fields (union printer variant) are declared.
 */
#[repr(C)]
struct DEVMODEW {
    dmDeviceName: [wchar_t; 32],
    dmSpecVersion: c_ushort,
    dmDriverVersion: c_ushort,
    dmSize: c_ushort,
    dmDriverExtra: c_ushort,
    dmFields: c_ulong,
    // Union: printer variant
    dmOrientation: c_short,
    dmPaperSize: c_short,
    dmPaperLength: c_short,
    dmPaperWidth: c_short,
    dmScale: c_short,
    dmCopies: c_short,
    dmDefaultSource: c_short,
    dmPrintQuality: c_short,
    // After union
    dmColor: c_short,
    dmDuplex: c_short,
    dmYResolution: c_short,
    dmTTOption: c_short,
    dmCollate: c_short,
    dmFormName: [wchar_t; 32],
    dmLogPixels: c_ushort,
    dmBitsPerPel: c_ulong,
    dmPelsWidth: c_ulong,
    dmPelsHeight: c_ulong,
    dmDisplayFlags: c_ulong,
    dmDisplayFrequency: c_ulong,
    dmICMMethod: c_ulong,
    dmICMIntent: c_ulong,
    dmMediaType: c_ulong,
    dmDitherType: c_ulong,
    dmReserved1: c_ulong,
    dmReserved2: c_ulong,
    dmPanningWidth: c_ulong,
    dmPanningHeight: c_ulong,
}

/**
 * Specifies a date and time, using individual members for the month, day, year, weekday, hour, minute, second, and millisecond.
 * https://learn.microsoft.com/en-us/windows/win32/api/minwinbase/ns-minwinbase-systemtime
 */
#[derive(Debug, Clone)]
#[repr(C)]
pub struct SYSTEMTIME {
    pub wYear: c_ushort,
    pub wMonth: c_ushort,
    pub wDayOfWeek: c_ushort,
    pub wDay: c_ushort,
    pub wHour: c_ushort,
    pub wMinute: c_ushort,
    pub wSecond: c_ushort,
    pub wMilliseconds: c_ushort,
}

/**
 * The JOB_INFO_1 structure specifies print-job information.
 * https://learn.microsoft.com/en-us/windows/win32/printdocs/job-info-1
 */
#[derive(Debug, Clone)]
#[repr(C)]
pub struct JOB_INFO_1W {
    JobId: c_ulong,
    pPrinterName: *mut wchar_t,
    pMachineName: *mut wchar_t,
    pUserName: *mut wchar_t,
    pDocument: *mut wchar_t,
    pDatatype: *mut wchar_t,
    pStatus: *mut wchar_t,
    Status: c_ulong,
    Priority: c_ulong,
    Position: c_ulong,
    TotalPages: c_ulong,
    PagesPrinted: c_ulong,
    Submitted: SYSTEMTIME,
}

impl PlatformPrinterJobGetters for JOB_INFO_1W {
    fn get_id(&self) -> u64 {
        self.JobId.into()
    }

    fn get_name(&self) -> String {
        wchar_t_to_string(self.pDocument)
    }

    fn get_state(&self) -> u64 {
        self.Status.into()
    }

    fn get_printer(&self) -> String {
        wchar_t_to_string(self.pPrinterName)
    }

    fn get_media_type(&self) -> String {
        wchar_t_to_string(self.pDatatype)
    }

    fn get_created_at(&self) -> std::time::SystemTime {
        calculate_system_time(
            self.Submitted.wYear,
            self.Submitted.wMonth,
            self.Submitted.wDay,
            self.Submitted.wHour,
            self.Submitted.wMinute,
            self.Submitted.wSecond,
            self.Submitted.wMilliseconds,
        )
    }

    fn get_processed_at(&self) -> Option<std::time::SystemTime> {
        Some(self.get_created_at())
    }

    fn get_completed_at(&self) -> Option<std::time::SystemTime> {
        Some(self.get_created_at())
    }
}

/**
 * Get the default DEVMODE for a printer via DocumentPropertiesW.
 * Returns the DEVMODE pointer and its Layout for deallocation.
 */
unsafe fn get_printer_devmode(
    printer_name: &str,
) -> Result<(*mut DEVMODEW, Layout), PrintersError> {
    let handle = open_printer(printer_name)?;
    let wide_name = str_to_wide_string(printer_name);

    let size = unsafe {
        DocumentPropertiesW(
            ptr::null_mut(),
            handle,
            wide_name.as_ptr() as *const wchar_t,
            ptr::null_mut(),
            ptr::null_mut(),
            0,
        )
    };

    if size < 0 {
        unsafe { ClosePrinter(handle) };
        return Err(PrintersError::print_error(
            "DocumentPropertiesW failed to get buffer size",
        ));
    }

    let layout = Layout::from_size_align(size as usize, align_of::<DEVMODEW>()).map_err(|_| {
        unsafe { ClosePrinter(handle) };
        PrintersError::print_error("Failed to create DEVMODE layout")
    })?;

    let devmode = unsafe { alloc(layout) } as *mut DEVMODEW;

    let result = unsafe {
        DocumentPropertiesW(
            ptr::null_mut(),
            handle,
            wide_name.as_ptr() as *const wchar_t,
            devmode as *mut c_void,
            ptr::null_mut(),
            DM_OUT_BUFFER,
        )
    };

    unsafe { ClosePrinter(handle) };

    if result < 0 {
        unsafe { dealloc(devmode as *mut u8, layout) };
        return Err(PrintersError::print_error(
            "DocumentPropertiesW failed to get DEVMODE",
        ));
    }

    Ok((devmode, layout))
}

/**
 * Open a printer, optionally with a DEVMODE applied via PRINTER_DEFAULTS.
 */
fn open_printer(printer_name: &str) -> Result<*mut c_void, PrintersError> {
    open_printer_with_devmode(printer_name, None)
}

fn open_printer_with_devmode(
    printer_name: &str,
    devmode: Option<*mut DEVMODEW>,
) -> Result<*mut c_void, PrintersError> {
    let printer_name = str_to_wide_string(printer_name);
    let mut printer_handle: *mut c_void = ptr::null_mut();

    let mut defaults = devmode.map(|dm| PrinterDefaultW {
        pDatatype: ptr::null_mut(),
        pDevMode: dm as *mut c_void,
        DesiredAccess: PRINTER_ACCESS_USE,
    });

    let p_defaults = match defaults.as_mut() {
        Some(d) => d as *mut PrinterDefaultW,
        None => ptr::null_mut(),
    };

    return if unsafe {
        OpenPrinterW(
            printer_name.as_ptr() as *const wchar_t,
            &mut printer_handle,
            p_defaults,
        )
    } == 0
    {
        Err(PrintersError::job_error("OpenPrinterW failed"))
    } else {
        Ok(printer_handle)
    };
}

/**
 * Print a buffer with winspool WritePrinter.
 *
 * Supported raw_properties:
 * - "copies"          : number of copies (numeric string)
 * - "document-format" : data type (default "RAW")
 * - "ColorModel"      : "Gray" | "Color"
 * - "sides"           : "one-sided" | "two-sided-long-edge" | "two-sided-short-edge"
 * - "PageSize"/"media": paper size name (e.g. "A4", "B5", "Letter") or numeric DMPAPER_* id
 * - "PageWidth"/"media-width": custom paper width in 1/10 mm (e.g. "1480" for 148mm)
 * - "PageHeight"/"media-height": custom paper height in 1/10 mm (e.g. "2100" for 210mm)
 * - "InputSlot"/"media-source": tray name (e.g. "Auto", "Tray1", "Tray2") or numeric DMBIN_* id
 *
 * Note: When both PageWidth and PageHeight are specified, they take precedence over PageSize.
 */
pub fn print_buffer(
    printer_name: &str,
    job_name: Option<&str>,
    buffer: &[u8],
    options: &[(&str, &str)],
) -> Result<u64, PrintersError> {
    unsafe {
        let mut copies: u32 = 1;
        let mut data_type = "RAW";

        let mut dm_paper_size: Option<c_short> = None;
        let mut dm_paper_width: Option<c_short> = None;
        let mut dm_paper_length: Option<c_short> = None;
        let mut dm_default_source: Option<c_short> = None;
        let mut dm_duplex: Option<c_short> = None;
        let mut dm_color: Option<c_short> = None;

        for option in options {
            match option.0 {
                "copies" => copies = option.1.parse().unwrap_or(copies),
                "document-format" => data_type = option.1,
                "ColorModel" => dm_color = parse_color_model(option.1),
                "sides" => dm_duplex = parse_duplex(option.1),
                "PageSize" | "media" => dm_paper_size = parse_paper_size(option.1),
                "InputSlot" | "media-source" => dm_default_source = parse_media_source(option.1),
                "PageWidth" | "media-width" => {
                    dm_paper_width = option.1.parse::<c_short>().ok();
                }
                "PageHeight" | "media-height" => {
                    dm_paper_length = option.1.parse::<c_short>().ok();
                }
                _ => {}
            }
        }

        // Custom paper size: width and height must both be specified
        let has_custom_size = dm_paper_width.is_some() && dm_paper_length.is_some();

        let has_devmode_options = dm_paper_size.is_some()
            || has_custom_size
            || dm_default_source.is_some()
            || dm_duplex.is_some()
            || dm_color.is_some();

        // Try to apply DEVMODE if any driver-level options were specified
        let devmode_state = if has_devmode_options {
            match get_printer_devmode(printer_name) {
                Ok((devmode, layout)) => {
                    macro_rules! apply {
                        ($value:expr, $field:ident, $flag:expr) => {
                            if let Some(v) = $value {
                                (*devmode).$field = v;
                                (*devmode).dmFields |= $flag;
                            }
                        };
                    }
                    if has_custom_size {
                        // Custom size: clear dmPaperSize, set width and length
                        (*devmode).dmPaperSize = 0;
                        (*devmode).dmFields |= DM_PAPERSIZE;
                        apply!(dm_paper_width, dmPaperWidth, DM_PAPERWIDTH);
                        apply!(dm_paper_length, dmPaperLength, DM_PAPERLENGTH);
                    } else {
                        apply!(dm_paper_size, dmPaperSize, DM_PAPERSIZE);
                    }
                    apply!(dm_default_source, dmDefaultSource, DM_DEFAULTSOURCE);
                    apply!(dm_duplex, dmDuplex, DM_DUPLEX);
                    apply!(dm_color, dmColor, DM_COLOR);
                    (*devmode).dmCopies = copies as c_short;
                    (*devmode).dmFields |= DM_COPIES;
                    Some((devmode, layout))
                }
                Err(_) => None, // Fall back to legacy behavior
            }
        } else {
            None
        };

        let devmode_ptr = devmode_state.as_ref().map(|(dm, _)| *dm);
        let printer_handle = match open_printer_with_devmode(printer_name, devmode_ptr) {
            Ok(handle) => handle,
            Err(e) => {
                free_devmode(devmode_state);
                return Err(e);
            }
        };

        let mut p_data_type = str_to_wide_string(data_type);
        let mut p_doc_name =
            str_to_wide_string(job_name.unwrap_or(get_current_epoch().to_string().as_str()));

        let doc_info = DocInfo1 {
            pDocName: p_doc_name.as_mut_ptr() as *mut wchar_t,
            pDatatype: p_data_type.as_mut_ptr() as *mut wchar_t,
            pOutputFile: ptr::null_mut(),
        };

        let job_id = StartDocPrinterW(printer_handle, 1, &doc_info);
        if job_id == 0 {
            ClosePrinter(printer_handle);
            free_devmode(devmode_state);
            return Err(PrintersError::job_error("StartDocPrinterW failed"));
        }

        // When DEVMODE is active, copies are handled by the driver (dmCopies).
        // Otherwise, fall back to writing the buffer multiple times.
        let write_count = if devmode_state.is_some() { 1 } else { copies };

        for _ in 0..write_count {
            if StartPagePrinter(printer_handle) != 0 {
                let mut bytes_written: c_ulong = 0;
                WritePrinter(
                    printer_handle,
                    buffer.as_ptr() as *mut c_void,
                    buffer.len() as c_ulong,
                    &mut bytes_written,
                );
                EndPagePrinter(printer_handle);
            }
        }

        EndDocPrinter(printer_handle);
        ClosePrinter(printer_handle);
        free_devmode(devmode_state);

        Ok(job_id as u64)
    }
}

/// Free a DEVMODE buffer if present.
unsafe fn free_devmode(state: Option<(*mut DEVMODEW, Layout)>) {
    if let Some((devmode, layout)) = state {
        unsafe { dealloc(devmode as *mut u8, layout) };
    }
}

/**
 * Retrieve print jobs of a specific printer with EnumJobsW
 */
pub fn enum_printer_jobs(printer_name: &str) -> Result<&'static [JOB_INFO_1W], PrintersError> {
    let printer_handle = open_printer(printer_name)?;

    let mut enum_result = 0;
    let mut buffer_ptr: *mut JOB_INFO_1W = ptr::null_mut();
    let mut jobs_count: c_ulong = 0;
    let mut bytes_needed: c_ulong = 0;

    for _ in 0..2 {
        enum_result = unsafe {
            EnumJobsW(
                printer_handle,
                0,
                0xFFFFFFFF,
                1,
                buffer_ptr as *mut c_void,
                bytes_needed,
                &mut bytes_needed,
                &mut jobs_count,
            )
        };

        if enum_result != 0 || bytes_needed == 0 {
            break;
        }

        buffer_ptr = alloc_s::<JOB_INFO_1W>(bytes_needed);
    }

    unsafe { ClosePrinter(printer_handle) };

    if enum_result == 0 {
        return Err(PrintersError::job_error("EnumJobsW failed"));
    }

    Ok(if jobs_count > 0 {
        unsafe { slice::from_raw_parts(buffer_ptr as *const JOB_INFO_1W, jobs_count as usize) }
    } else {
        &[]
    })
}

/**
 * Change job state
 */
pub fn set_job_state(printer_name: &str, command: u64, job_id: u64) -> Result<(), PrintersError> {
    unsafe {
        let printer_handle = open_printer(printer_name)?;

        let result = SetJobW(
            printer_handle,
            job_id as c_ulong,
            0,
            ptr::null_mut(),
            command as c_ulong,
        );

        ClosePrinter(printer_handle);

        if result == 0 {
            Err(PrintersError::job_error("SetJobW failed"))
        } else {
            Ok(())
        }
    }
}
