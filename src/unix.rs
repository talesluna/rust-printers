use cups::dests::get_dests;
use std::str;

use crate::common::{
    base::{
        job::{PrinterJob, PrinterJobOptions, PrinterJobState},
        printer::{Printer, PrinterState},
    },
    traits::platform::{PlatformActions, PlatformPrinterGetters},
};

mod cups;
mod utils;

impl PlatformActions for crate::Platform {
    fn get_printers() -> Vec<Printer> {
        let dests = get_dests().unwrap_or_default();
        let printers = dests
            .iter()
            .map(|p| Printer::from_platform_printer_getters(p))
            .collect();

        cups::dests::free(dests);
        printers
    }

    fn print(
        printer_system_name: &str,
        buffer: &[u8],
        options: PrinterJobOptions
    ) -> Result<u64, &'static str> {
        let path = utils::file::save_tmp_file(buffer);
        if path.is_some() {
            let file_path = path.unwrap();
            Self::print_file(
                printer_system_name,
                file_path.to_str().unwrap(),
                options
            )
        } else {
            Err("Failed to create temp file")
        }
    }

    fn print_file(
        printer_system_name: &str,
        file_path: &str,
        options: PrinterJobOptions
    ) -> Result<u64, &'static str> {
        cups::jobs::print_file(
            printer_system_name, 
            file_path, 
            options.name, 
            options.raw_properties
        )
    }

    fn get_printer_jobs(printer_name: &str, active_only: bool) -> Vec<PrinterJob> {
        cups::jobs::get_printer_jobs(printer_name, active_only)
            .unwrap_or_default()
            .iter()
            .map(|j| PrinterJob::from_platform_printer_job_getters(j))
            .collect()
    }

    fn get_default_printer() -> Option<Printer> {
        let dests = get_dests().unwrap_or_default();
        let dest = dests
            .iter()
            .find(|d| d.get_is_default())
            .map(|d| Printer::from_platform_printer_getters(d));

        cups::dests::free(dests);
        dest
    }

    fn get_printer_by_name(printer_name: &str) -> Option<Printer> {
        let dests = get_dests().unwrap_or_default();
        let dest = dests
            .iter()
            .find(|d| d.get_name() == printer_name || d.get_system_name() == printer_name)
            .map(|d| Printer::from_platform_printer_getters(d));

        cups::dests::free(dests);
        dest
    }

    fn parse_printer_state(platform_state: u64, state_reasons: &str) -> PrinterState {
        if state_reasons.contains("offline-report") {
            return PrinterState::OFFLINE;
        }

        match platform_state {
            3 => PrinterState::READY,
            4 => PrinterState::PRINTING,
            5 => PrinterState::PAUSED,
            _ => PrinterState::UNKNOWN,
        }
    }

    fn parse_printer_job_state(platform_state: u64) -> PrinterJobState {
        match platform_state {
            3 => PrinterJobState::PENDING,
            4 | 6 => PrinterJobState::PAUSED,
            5 => PrinterJobState::PROCESSING,
            7 | 8 => PrinterJobState::CANCELLED,
            9 => PrinterJobState::COMPLETED,
            _ => PrinterJobState::UNKNOWN,
        }
    }
}
