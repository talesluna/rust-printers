use crate::common::{
    base::{
        job::{PrinterJob, PrinterJobOptions, PrinterJobState},
        printer::Printer,
        printer::PrinterState,
    },
    traits::platform::{PlatformActions, PlatformPrinterGetters},
    utils::file,
};

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
        options: PrinterJobOptions,
    ) -> Result<u64, String> {
        let buffer = options.converter.convert(buffer)?;
        let buffer = &buffer.as_slice();

        winspool::jobs::print_buffer(
            printer_system_name,
            options.name,
            buffer,
            options.raw_properties,
        )
    }

    fn print_file(
        printer_system_name: &str,
        file_path: &str,
        options: PrinterJobOptions,
    ) -> Result<u64, String> {
        let buffer = file::get_file_as_bytes(file_path)?;
        Self::print(printer_system_name, &buffer, options)
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

    fn set_job_state(
        printer_name: &str,
        job_id: u64,
        state: PrinterJobState,
    ) -> Result<(), String> {
        return match state {
            PrinterJobState::PAUSED => winspool::jobs::set_job_state(printer_name, 1, job_id),
            PrinterJobState::PENDING => winspool::jobs::set_job_state(printer_name, 4, job_id),
            PrinterJobState::CANCELLED => winspool::jobs::set_job_state(printer_name, 5, job_id),
            PrinterJobState::PROCESSING => winspool::jobs::set_job_state(printer_name, 2, job_id),
            _ => Err("Operation canot be defined".into()),
        };
    }
}
