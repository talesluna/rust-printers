use std::path::Path;

use crate::common::base::job::PrinterJobState;
use crate::common::base::printer::PrinterState;
use crate::common::base::{job::PrinterJob, printer::Printer};
use crate::common::traits::platform::{PlatformActions, PlatformPrinterGetters};

mod utils;
mod winspool;

impl PlatformActions for crate::Platform {
    fn get_printers() -> Vec<Printer> {
        let data = winspool::info::enum_printers(None);

        let printers: Vec<Printer> = data
            .iter()
            .filter(|p| !p.pPrinterName.is_null())
            .map(|p| Printer::from_platform_printer_getters(p))
            .collect();

        winspool::info::free(data);
        return printers;
    }

    fn print(
        printer_system_name: &str,
        buffer: &[u8],
        job_name: Option<&str>,
    ) -> Result<(), &'static str> {
        return winspool::jobs::print_buffer(printer_system_name, job_name, buffer);
    }

    fn print_file(
        printer_system_name: &str,
        file_path: &str,
        job_name: Option<&str>,
    ) -> Result<(), &'static str> {
        let buffer = utils::file::get_file_as_bytes(file_path);
        return if buffer.is_some() {
            let job_name =
                job_name.unwrap_or(Path::new(file_path).file_name().unwrap().to_str().unwrap());
            return Self::print(printer_system_name, &buffer.unwrap(), Some(job_name));
        } else {
            Err("failed to read file")
        };
    }

    fn get_printer_jobs(printer_name: &str, active_only: bool) -> Vec<PrinterJob> {
        return winspool::jobs::enum_printer_jobs(printer_name)
            .unwrap_or_default()
            .into_iter()
            .map(|j| PrinterJob::from_platform_printer_job_getters(j))
            .filter(|j| {
                return if active_only {
                    j.state == PrinterJobState::PENDING
                        || j.state == PrinterJobState::PROCESSING
                        || j.state == PrinterJobState::PAUSED
                } else {
                    true
                };
            })
            .collect();
    }

    fn get_default_printer() -> Option<Printer> {
        return winspool::info::get_default_printer()
            .map(|p| Printer::from_platform_printer_getters(p));
    }

    fn get_printer_by_name(name: &str) -> Option<Printer> {
        return winspool::info::enum_printers(None)
            .into_iter()
            .find(|p| p.get_name() == name || p.get_system_name() == name)
            .map(|p| Printer::from_platform_printer_getters(p));
    }

    fn parse_printer_state(platform_state: &str) -> PrinterState {
        if platform_state == "0" {
            return PrinterState::READY;
        }

        if platform_state == "1" || platform_state == "2" {
            return PrinterState::PAUSED;
        }

        if platform_state == "5" {
            return PrinterState::PRINTING;
        }

        return PrinterState::UNKNOWN;
    }

    fn parse_printer_job_state(platform_state: u64) -> PrinterJobState {
        if platform_state == 32
            || platform_state == 64
            || platform_state == 512
            || platform_state == 1024
        {
            return PrinterJobState::PENDING;
        }

        if platform_state == 1 || platform_state == 8 {
            return PrinterJobState::PAUSED;
        }

        if platform_state == 16 || platform_state == 2048 || platform_state == 8192 {
            return PrinterJobState::PROCESSING;
        }

        if platform_state == 4 || platform_state == 256 {
            return PrinterJobState::CANCELLED;
        }

        if platform_state == 128 || platform_state == 4096 {
            return PrinterJobState::COMPLETED;
        }

        return PrinterJobState::UNKNOWN;
    }
}
