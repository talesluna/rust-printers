//! Get printers and print files or bytes on unix and windows
//!
//! Printers is a simple lib for running "native" printing commands in unix *(lp/lpstat)* and windows *(lpr/wmic)* systems.
//! Printer can provide a list of printers available on the system and perform document printing.
//!
//! ```rust
//! use printers;
//!
//! let printers = printers::get_printers();
//!
//! for printer in printers {
//!     let job1 = printer.print("42".as_bytes());
//!     let job2 = printer.print_file("/path/to/any.file");
//!
//!     println!("{:?}", printer);
//!     println!("{:?}", job1);
//!     println!("{:?}", job2);
//! }
//! ```
//!
//!
use std::env;
use std::fs::File;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

// #[cfg(target_family = "unix")]
mod unix;

// #[cfg_attr(rust_analyzer, allow(dead_code))]
// #[cfg(target_family = "windows")]
mod windows;

/// Printer and Job control
pub mod printer;
pub mod shared;

/**
 * Print bytes on specific printer
 */
pub fn print(printer_name: &str, buffer: &[u8]) -> Result<bool, String> {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();

    let tmp_file_path = env::temp_dir().join(time.to_string());

    let mut tmp_file = File::create(&tmp_file_path).unwrap();
    let save = tmp_file.write(buffer);

    if save.is_err() {
        let error = save.err().unwrap();
        return Result::Err(error.to_string())
    }

    return print_file(printer_name, tmp_file_path.to_str().unwrap());

}

/**
 * Print specific file on a specific printer
 */
pub fn print_file(printer_name: &str, file_path: &str) -> Result<bool, String> {
    return unix::print(printer_name, file_path);
}

/**
 * Return all available printers on system
 */
pub fn get_printers() -> Vec<printer::Printer> {
    #[cfg_attr(rust_analyzer, allow(dead_code))]
    #[cfg(target_family = "windows")]
    return windows::get_printers();

    #[cfg(target_family = "unix")]
    return unix::get_printers();

    #[cfg(target_family = "wasm")]
    panic!("Unsupported OS");
}

/**
 * If you known the printer Name you can try get the printer directly from they
 */
pub fn get_printer_by_name(name: &str) -> Option<printer::Printer> {
    let printers = get_printers();

    let opt = printers.iter().find(|&printer| {
        return printer.clone().name.eq(name) || printer.clone().system_name.eq(name);
    });

    return opt.cloned();
}
