mod unix;
mod windows;
mod printer;

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
 * Return all printers by system
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
