use crate::{printer::{Printer, PrinterState}, shared::interface::PlatformPrinterGetters};
use std::process::Command;

mod winspool;

/**
 * Get printers on windows using winspool
 */
pub fn get_printers() -> Vec<Printer> {

    let available_printers = &winspool::enum_printers();
    let mut printers = Vec::<Printer>::new();

    for printer in available_printers {

        let mut state = crate::printer::PrinterState::UNKNOWN;
        let winspool_state = printer.get_state();

        if winspool_state == "0" {
            state = PrinterState::READY;
        }

        if winspool_state == "1" || winspool_state == "2" {
            state = PrinterState::PAUSED;
        }

        if winspool_state == "5" {
            state = PrinterState::PRINTING;
        }

        printers.push(Printer::from_platform_printer_getters(printer, state));
    }

    return printers;

}

/**
 * Print on windows systems using winspool
 */
pub fn print(printer_system_name: &str, file_path: &str, job_name: Option<&str>) -> Result<bool, String> {
    // let result = lpr::add_job("123".as_bytes(), printer_system_name, job_name);
    let job_name = job_name.unwrap_or(file_path);
    let status = Command::new("powershell")
    .args(&[
        "-Command",
        &format!(
            "Start-Job -ScriptBlock {{ Get-Content '{}' | Out-Printer -Name '{}' }} -Name '{}' *> $null; Wait-Job -Name '{}' | Receive-Job *> $null",
            file_path, printer_system_name, job_name, job_name
        ),
    ])
    .spawn();

    return if status.is_ok() {
        Result::Ok(true)
    } else {
        Result::Err("failure to send document to printer".to_string())
    }
}

