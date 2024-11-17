use std::str;
use crate::common::{
    printer::{job::PrinterJob, Printer},
    traits::platform::PlatformActions
};

mod cups;

impl PlatformActions for crate::Platform {
    fn get_printers() -> Vec<Printer> {
        let dests = cups::dests::get_dests();
        let mut printers = Vec::<Printer>::new();

        for platform_printer in dests {
            if !platform_printer.is_shared_duplex() {
                printers.push(Printer::from_platform_printer_getters(platform_printer));
            }
        }

        cups::dests::free(dests);
        return printers;
    }

    fn print(printer_system_name: &str, file_path: &str, job_name: Option<&str>) -> bool {
        let result = cups::jobs::print_file(printer_system_name, file_path, job_name);
        return result;
    }

    fn get_printer_jobs(printer_name: &str, active_only: bool) -> Vec<PrinterJob> {
        let printer_jobs = cups::jobs::get_printer_jobs(printer_name, active_only);
        let mut jobs = Vec::new();
        for platform_printer_job in printer_jobs {
            jobs.push(PrinterJob::from_platform_printer_job_getters(
                platform_printer_job,
            ));
        }

        return jobs;
    }

    fn get_default_printer() -> Option<Printer> {
        return None;
    }

    fn get_printer_by_name(printer_name: &str) -> Option<Printer> {
        return None;
    }
}
