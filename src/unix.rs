use cups::dests::get_dests;
use std::str;

use crate::common::{
    base::{
        errors::PrintersError,
        job::{PrinterJob, PrinterJobOptions, PrinterJobState},
        printer::{Printer, PrinterState},
    },
    converters::Converter,
    traits::platform::{PlatformActions, PlatformPrinterGetters},
    utils::file,
};

mod cups;
mod utils;

impl PlatformActions for crate::Platform {
    fn get_printers() -> Vec<Printer> {
        if let Some(dests) = get_dests() {
            let printers = dests
                .iter()
                .map(|p| Printer::from_platform_printer_getters(p))
                .collect();

            cups::dests::free(dests);
            printers
        } else {
            Vec::new()
        }
    }

    fn print(
        printer_system_name: &str,
        buffer: &[u8],
        options: PrinterJobOptions,
    ) -> Result<u64, PrintersError> {
        let buffer = options.converter.convert(buffer)?;
        let buffer = &buffer.as_slice();
        let file_path = file::save_tmp_file(buffer)?;

        cups::jobs::print_file(
            printer_system_name,
            file_path.to_str().unwrap_or_default(),
            options.name,
            options.raw_properties,
        )
    }

    fn print_file(
        printer_system_name: &str,
        file_path: &str,
        options: PrinterJobOptions,
    ) -> Result<u64, PrintersError> {
        if options.converter != Converter::None {
            let buffer = file::get_file_as_bytes(file_path)?;
            return Self::print(printer_system_name, buffer.as_slice(), options);
        }

        cups::jobs::print_file(
            printer_system_name,
            file_path,
            options.name,
            options.raw_properties,
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
        let dests = get_dests()?;
        let printer = dests
            .iter()
            .find(|d| d.get_is_default())
            .map(|d| Printer::from_platform_printer_getters(d));

        cups::dests::free(dests);
        printer
    }

    fn get_printer_by_name(printer_name: &str) -> Option<Printer> {
        let dests = get_dests()?;
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

    fn set_job_state(
        printer_name: &str,
        job_id: u64,
        state: PrinterJobState,
    ) -> Result<(), PrintersError> {
        let result = match state {
            PrinterJobState::PENDING => cups::jobs::restart_job(printer_name, job_id as i32),
            PrinterJobState::PROCESSING => cups::jobs::release_job(printer_name, job_id as i32),
            PrinterJobState::PAUSED => cups::jobs::hold_job(printer_name, job_id as i32),
            PrinterJobState::CANCELLED => cups::jobs::cancel_job(printer_name, job_id as i32),
            _ => false,
        };

        if result {
            Ok(())
        } else {
            Err(PrintersError::print_error("cups method failed"))
        }
    }
}
