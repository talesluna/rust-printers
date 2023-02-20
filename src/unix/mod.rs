use std::str;
use crate::printer::{Printer, PrinterState};

mod cups;

/**
 * Get printers on unix systems using CUPS
 */
pub fn get_printers() -> Vec<Printer> {

    let cups_dests = cups::get_dests();
    let mut printers: Vec<Printer> = vec![];

    for dest in cups_dests {

        let mut state = crate::printer::PrinterState::PAUSED;
        let cups_state = dest.get_state();

        if cups_state == "3" {
            state = PrinterState::READY;
        }
        if cups_state == "4" {
            state = PrinterState::PRINTING;
        }

        printers.push(
            Printer {
                name: dest.get_printer_info(),
                system_name: dest.get_name(),
                driver_name: dest.get_marker_and_model(),
                location: dest.get_location(),
                state,
                uri: dest.get_uri(),
                is_default: dest.get_is_default().is_positive(),
                is_shared: dest.get_is_shared() == "true",
            }
        );
    }

    // cups::free_dests(cups_dests);
    return printers;
}

/**
 * Print on unix systems using CUPS
 */
pub fn print(printer_system_name: &str, file_path: &str) -> Result<bool, String> {
    let result = cups::print_file(printer_system_name, file_path);
    if result {
        return Result::Ok(true);
    } else {
        return Result::Err("failure on send document to printer".to_string());
    }
}
