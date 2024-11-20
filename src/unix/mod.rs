use std::str;
use cups::dests::get_dests;

use crate::common::{
    base::{job::PrinterJob, printer::Printer},
    traits::platform::{PlatformActions, PlatformPrinterGetters}
};

mod cups;

impl PlatformActions for crate::Platform {

    fn get_printers() -> Vec<Printer> {
        let dests = cups::dests::get_dests().unwrap_or_default();
        let printers = dests
            .into_iter()
            .filter(|p|!p.is_shared_duplex())
            .map(|p|Printer::from_platform_printer_getters(p))
            .collect();

        cups::dests::free(dests);
        return printers;
    }

    #[cfg(target_family = "unix")]
    fn print(printer_system_name: &str, file_path: &str, job_name: Option<&str>) ->  Result<(), &'static str> {
        return cups::jobs::print_file(printer_system_name, file_path, job_name);
    }

    fn get_printer_jobs(printer_name: &str, active_only: bool) -> Vec<PrinterJob> {
        return cups::jobs::get_printer_jobs(printer_name, active_only)
            .unwrap_or_default()
            .into_iter()
            .map(|j|PrinterJob::from_platform_printer_job_getters(j))
            .collect();
    }

    fn get_default_printer() -> Option<Printer> {
        let dests = get_dests().unwrap_or_default();
        let dest = dests
            .into_iter()
            .find(|d| d.get_is_default())
            .map(|d|Printer::from_platform_printer_getters(d));

        cups::dests::free(dests);
        return dest;
    }

    fn get_printer_by_name(printer_name: &str) -> Option<Printer> {
        let dests = get_dests().unwrap_or_default();
        let dest = dests
            .into_iter()
            .find(|d| d.get_name() == printer_name || d.get_system_name() == printer_name)
            .map(|d|Printer::from_platform_printer_getters(d));

        cups::dests::free(dests);
        return dest;
    }
}
