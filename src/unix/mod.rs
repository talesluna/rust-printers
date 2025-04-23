use cups::dests::get_dests;
use std::str;

use crate::common::{
    base::{
        job::{PrinterJob, PrinterJobState},
        printer::{Printer, PrinterState},
    },
    traits::platform::{PlatformActions, PlatformPrinterGetters},
};

mod cups;
mod utils;

impl PlatformActions for crate::Platform {
    fn get_printers() -> Vec<Printer> {
        let dests = cups::dests::get_dests().unwrap_or_default();
        let printers = dests
            .into_iter()
            .filter(|p| !p.is_shared_duplex())
            .map(|p| Printer::from_platform_printer_getters(p))
            .collect();

        cups::dests::free(dests);
        printers
    }

    fn print(
        printer_system_name: &str,
        buffer: &[u8],
        job_name: Option<&str>,
    ) -> Result<(), &'static str> {
        let path = crate::unix::utils::file::save_tmp_file(buffer);
        if path.is_some() {
            let file_path = path.unwrap();
            Self::print_file(printer_system_name, file_path.to_str().unwrap(), job_name)
        } else {
            Err("Failed to create temp file")
        }
    }

    fn print_file(
        printer_system_name: &str,
        file_path: &str,
        job_name: Option<&str>,
    ) -> Result<(), &'static str> {
        cups::jobs::print_file(printer_system_name, file_path, job_name)
    }

    fn get_printer_jobs(printer_name: &str, active_only: bool) -> Vec<PrinterJob> {
        cups::jobs::get_printer_jobs(printer_name, active_only)
            .unwrap_or_default()
            .into_iter()
            .map(|j| PrinterJob::from_platform_printer_job_getters(j))
            .collect()
    }

    fn get_default_printer() -> Option<Printer> {
        let dests = get_dests().unwrap_or_default();
        let dest = dests
            .into_iter()
            .find(|d| d.get_is_default())
            .map(|d| Printer::from_platform_printer_getters(d));

        cups::dests::free(dests);
        dest
    }

    fn get_printer_by_name(printer_name: &str) -> Option<Printer> {
        let dests = get_dests().unwrap_or_default();
        let dest = dests
            .into_iter()
            .find(|d| d.get_name() == printer_name || d.get_system_name() == printer_name)
            .map(|d| Printer::from_platform_printer_getters(d));

        cups::dests::free(dests);
        dest
    }

    fn parse_printer_state(platform_state: &str) -> PrinterState {
        if platform_state == "3" {
            return PrinterState::READY;
        }

        if platform_state == "4" {
            return PrinterState::PRINTING;
        }

        if platform_state == "5" {
            return PrinterState::PAUSED;
        }

        PrinterState::UNKNOWN
    }

    fn parse_printer_job_state(platform_state: u64) -> PrinterJobState {
        if platform_state == 3 {
            return PrinterJobState::PENDING;
        }

        if platform_state == 4 || platform_state == 6 {
            return PrinterJobState::PAUSED;
        }

        if platform_state == 5 {
            return PrinterJobState::PROCESSING;
        }

        if platform_state == 7 || platform_state == 8 {
            return PrinterJobState::CANCELLED;
        }

        if platform_state == 9 {
            return PrinterJobState::COMPLETED;
        }

        PrinterJobState::UNKNOWN
    }
}
