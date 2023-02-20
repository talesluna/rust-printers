#[cfg_attr(analyzer, allow(dead_code))]

use crate::{printer::{Printer, PrinterState}, shared::interface::PlatformPrinterGetters};
use std::process::Command;

pub mod winspool;

/**
 * Get printers on windows using wmic
 */
pub fn get_printers() -> Vec<Printer> {

    let available_printers = winspool::enum_printers();
    let mut printers = Vec::<Printer>::new();

    for printer in available_printers {

        let mut state = crate::printer::PrinterState::PAUSED;
        let cups_state = printer.get_state();

        if cups_state == 0x00002000.to_string() {
            state = PrinterState::READY;
        }
        if cups_state == 0x00000400.to_string() {
            state = PrinterState::PRINTING;
        }

        printers.push(
            Printer {
                name: printer.get_name(),
                system_name: printer.get_name(),
                driver_name: printer.get_marker_and_model(),
                location: printer.get_location(),
                state,
                uri: printer.get_uri(),
                is_default: printer.get_is_default().is_positive(),
                is_shared: printer.get_is_shared() == "true",
            }
        );
    }

    return printers;

}

/**
 * Print on windows using lpr
 */
pub fn print(printer_system_name: &str, file_path: &str) -> Result<bool, String> {
    let child = Command::new("powershell")
        .arg(format!(
            "Get-Content -Path \"{}\" |  Out-Printer -Name \"{}\"",
            file_path, printer_system_name
        ))
        .spawn()
        .unwrap();

    if child.id() > 0 {
        return Result::Ok(true);
    }

    return Result::Err("Failure to start print process".to_string());
}
