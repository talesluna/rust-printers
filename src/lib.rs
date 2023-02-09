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
mod unix;
mod windows;

/// Printer and Job control
pub mod printer;

/**
 * Print bytes on specific printer
 */
pub fn print(printer: &printer::Printer, buffer: &[u8]) -> printer::Job {
    return printer.print(buffer);
}


/**
 * Print specific file on a specific printer
 */
pub fn print_file(printer: &printer::Printer, file_path: &str) -> printer::Job {
    return printer.print_file(file_path);
}


/**
 * Return all available printers on system
 */
pub fn get_printers() -> Vec<printer::Printer> {
    if cfg!(windows) {
        return windows::get_printers();
    }

    if cfg!(unix) {
        return unix::get_printers();
    }

    panic!("Unsupported OS");
}

/**
 * If you known the printer ID you can try get the printer directly from they
 */
pub fn get_printer_by_id(id: &str) -> Option<printer::Printer> {

    let printers = get_printers();

    let opt = printers.iter().find(|&printer| {
        return printer.clone().id.eq(id)
    });

    return opt.cloned();
}


/**
 * If you known the printer Name you can try get the printer directly from they
 */
pub fn get_printer_by_name(name: &str) -> Option<printer::Printer> {

    let printers = get_printers();

    let opt = printers.iter().find(|&printer| {
        return printer.clone().name.eq(name) || printer.clone().system_name.eq(name)
    });

    return opt.cloned();
}
