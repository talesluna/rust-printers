//! Get printers and send files or bytes to print on unix and windows
//!
//! Printers is a simple lib to call printers apis for unix *(cups)* and windows *(winspool)* systems.
//!
//! Printers can provide a list of printers available on the system and send print jobs to them
//!
//! ```rust
//! use printers::{get_printer_by_name, get_default_printer, get_printers};
//!
//! // Iterate all available printers
//! for printer in get_printers() {
//!     println!("{:?}", printer);
//! }
//!
//! // Get a printer by the name
//! let my_printer = get_printer_by_name("my_printer");
//! if my_printer.is_some() {
//!     let job_id = my_printer.unwrap().print_file("notes.txt", None);
//!     // Err("cupsPrintFile failed")
//! }
//!
//! // Use the default printer
//! let default_printer = get_default_printer();
//! if default_printer.is_some() {
//!     let job_id = default_printer.unwrap().print("my content".as_bytes(), Some("My Job"));
//! }
//! ```

struct Platform;

pub mod common;

#[cfg(target_family = "unix")]
mod unix;

#[cfg(target_family = "windows")]
mod windows;

use common::{base::printer::Printer, traits::platform::PlatformActions};

/**
 * Return all available printers on a system
 */
pub fn get_printers() -> Vec<Printer> {
    Platform::get_printers()
}

/**
 * If you know the printer name, you can try to get the printer directly
 */
pub fn get_printer_by_name(printer_name: &str) -> Option<Printer> {
    Platform::get_printer_by_name(printer_name)
}

/**
 * Return the default system printer
 */
pub fn get_default_printer() -> Option<Printer> {
    Platform::get_default_printer()
}
