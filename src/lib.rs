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
//!     let job_id = my_printer.unwrap().print_file("notes.txt", None, &[]);
//!     // Err("...") or Ok(u64)
//! }
//!
//! // Use the default printer
//! let default_printer = get_default_printer();
//! if default_printer.is_some() {
//!     // options are currently UNIX-only. see https://www.cups.org/doc/options.html
//!     let options = [
//!         ("document-format", "application/vnd.cups-raw"),
//!         ("copies", "2"),
//!     ];
//!     let job_id = default_printer.unwrap().print("my content".as_bytes(), Some("My Job"), &options);
//!     // Err("...") or Ok(u64)
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

#[cfg(target_family = "unix")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zpl_with_cups_options() {
        //println!("{:?}", get_printers());
        let printer = get_printer_by_name("Zebra Technologies ZTC ZD410-203dpi ZPL").unwrap();
        let zpl = "^XA~TA000~JSN^LT0^MNW^MTD^PON^PMN^LH0,0^JMA^PR6,6~SD15^JUS^LRN^CI27^XZ
^XA
^MMT
^PW399
^LL0216
^LS0
^FT50,85^A0N,24,20^FH^FDID: 1234^FS
^FT50,60^A0N,24,20^FH^FDLast name: Doe^FS
^FT50,35^A0N,24,20^FH^FDFirst name: John^FS
^PQ1,0,1,Y^XZ";
        let options = [
            ("document-format", "application/vnd.cups-raw"),
            ("copies", "2"),
        ];
        let job_id = printer.print(zpl.as_bytes(), Some("My ZPL Job"), &options);
        println!("Job ID: {:?}", job_id);
        // println!("{:?}", printer.get_job_history());
        assert!(job_id.is_ok());
    }
}

#[cfg(target_family = "windows")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zpl_with_win_options() {
        //println!("{:?}", get_printers());
        let printer = get_printer_by_name("Microsoft Print to PDF").unwrap();
        let zpl = "^XA~TA000~JSN^LT0^MNW^MTD^PON^PMN^LH0,0^JMA^PR6,6~SD15^JUS^LRN^CI27^XZ
^XA
^MMT
^PW399
^LL0216
^LS0
^FT50,85^A0N,24,20^FH^FDID: 1234^FS
^FT50,60^A0N,24,20^FH^FDLast name: Doe^FS
^FT50,35^A0N,24,20^FH^FDFirst name: John^FS
^PQ1,0,1,Y^XZ";
        let options = []; // currently no options are supported with Windows
        let job_id = printer.print(zpl.as_bytes(), Some("My ZPL Job"), &options);
        println!("Job ID: {:?}", job_id);
        // println!("{:?}", printer.get_job_history());
        assert!(job_id.is_ok());
    }
}
