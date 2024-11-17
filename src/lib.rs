//! Get printers and print files or bytes on unix and windows
//!
//! Printers **is not a lib for printer drivers or cups**. Printers is a simple lib to call printers apis for unix *(cups)* and windows *(winspool)* systems.
//! Printer can provide a list of printers available on the system and perform document printing.
//!
//! ```rust
//! use printers;
//!
//! let printers = printers::get_printers();
//!
//! for printer in printers {
//!     let job1 = printer.print("42".as_bytes(), Some("Everything"));
//!     let job2 = printer.print_file("/path/to/any.file", None);
//!
//!     println!("{:?}", printer);
//!     println!("{:?}", job1);
//!     println!("{:?}", job2);
//! }
//! ```
//!
//!

struct Platform;

pub mod common;

#[cfg(target_family = "unix")]
mod unix;

#[cfg(target_family = "windows")]
mod windows;

use common::{traits::platform::PlatformActions, printer::Printer};

/**
 * Return all available printers on system
 */
pub fn get_printers() -> Vec<Printer> {
    return Platform::get_printers();
}

/**
 * If you known the printer nme you can try get the printer directly
 */
pub fn get_printer_by_name(printer_name: &str) -> Option<Printer> {
    return Platform::get_printer_by_name(printer_name);
}

/**
 * Return the default system printer
 */
pub fn get_default_printer() -> Option<Printer> {
    return Platform::get_default_printer();
}