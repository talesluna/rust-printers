use crate::common::base::{job::PrinterJob, printer::Printer};
use crate::common::traits::platform::{PlatformActions, PlatformPrinterGetters};

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

    #[cfg(target_family = "windows")]
    fn print(printer_system_name: &str, buffer: &[u8], job_name: Option<&str>) -> Result<(), &'static str> {
        let job_name = job_name.unwrap_or("job");

        return winspool::jobs::print_to_printer(
            printer_system_name,
            job_name,
            buffer,
        );
    }

    // TODO: implements real logic with winspool
    fn get_printer_jobs(printer_name: &str, active_only: bool) -> Vec<PrinterJob> {
        return if !printer_name.is_empty() && active_only {
            Vec::new()
        } else {
            Vec::new()
        }
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
    
}
