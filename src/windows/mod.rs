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
        printers
    }

    fn print(
        printer_system_name: &str,
        buffer: &[u8],
        job_name: Option<&str>,
        options: &[(&str, &str)],
    ) -> Result<u64, &'static str> {
        winspool::jobs::print_buffer(printer_system_name, job_name, buffer, options)
    }

    fn print_file(
        printer_system_name: &str,
        file_path: &str,
        job_name: Option<&str>,
        options: &[(&str, &str)],
    ) -> Result<u64, &'static str> {
        let buffer = utils::file::get_file_as_bytes(file_path);
        if buffer.is_some() {
            let job_name =
                job_name.unwrap_or(Path::new(file_path).file_name().unwrap().to_str().unwrap());
            Self::print(
                printer_system_name,
                &buffer.unwrap(),
                Some(job_name),
                options,
            )
        } else {
            Err("failed to read file")
        }
    }

    fn get_printer_jobs(printer_name: &str, active_only: bool) -> Vec<PrinterJob> {
        winspool::jobs::enum_printer_jobs(printer_name)
            .unwrap_or_default()
            .into_iter()
            .map(|j| PrinterJob::from_platform_printer_job_getters(j))
            .filter(|j| {
                if active_only {
                    j.state == PrinterJobState::PENDING
                        || j.state == PrinterJobState::PROCESSING
                        || j.state == PrinterJobState::PAUSED
                } else {
                    true
                }
            })
            .collect()
    }

    fn get_default_printer() -> Option<Printer> {
        winspool::info::get_default_printer().map(|p| Printer::from_platform_printer_getters(p))
    }

    fn get_printer_by_name(name: &str) -> Option<Printer> {
        winspool::info::enum_printers(None)
            .into_iter()
            .find(|p| p.get_name() == name || p.get_system_name() == name)
            .map(|p| Printer::from_platform_printer_getters(p))
    }

    fn parse_printer_state(platform_state: u64, state_reasons: &str) -> PrinterState {
        if state_reasons.contains("offline") || state_reasons.contains("pending_deletion") {
            return PrinterState::OFFLINE;
        }

        match platform_state {
            s if s == 0 || s & (0x00000100 | 0x00004000) != 0 => PrinterState::READY,
            s if s & 0x00000400 != 0 => PrinterState::PRINTING,
            s if s & (0x00000001 | 0x00000002 | 0x00000008 | 0x00000010 | 0x00000020) != 0 => {
                PrinterState::PAUSED
            }
            s if s & (0x00000080 | 0x00400000 | 0x00001000 | 0x00000004) != 0 => {
                PrinterState::OFFLINE
            }
            _ => PrinterState::UNKNOWN,
        }
    }

    fn parse_printer_job_state(platform_state: u64) -> PrinterJobState {
        match platform_state {
            1 | 8 => PrinterJobState::PAUSED,
            4 | 256 => PrinterJobState::CANCELLED,
            16 | 2048 | 8192 => PrinterJobState::PROCESSING,
            32 | 64 | 512 | 1024 => PrinterJobState::PENDING,
            128 | 496 => PrinterJobState::COMPLETED,
            _ => PrinterJobState::UNKNOWN,
        }
    }
}
