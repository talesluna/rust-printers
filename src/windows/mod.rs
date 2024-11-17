use crate::common::printer::job::PrinterJob;
use crate::common::printer::Printer;
use crate::common::traits::platform::PlatformActions;

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

    fn print(printer_system_name: &str, file_path: &str, job_name: Option<&str>) -> bool {
        let job_name = job_name.unwrap_or(file_path);
        return winspool::jobs::print_to_printer(
            printer_system_name,
            file_path,
            job_name,
            "42".as_bytes(),
        );
    }

    fn get_printer_jobs(printer_name: &str, active_only: bool) -> Vec<PrinterJob> {
        return Vec::new();
    }

    fn get_default_printer() -> Option<Printer> {
        return winspool::info::get_default_printer()
            .map(|p| Printer::from_platform_printer_getters(p));
    }

    fn get_printer_by_name(name: &str) -> Option<Printer> {
        let result = winspool::info::enum_printers(Some(name));
        println!(">>> {:}", result.len());
        return result
            .get(0)
            .map(|p| Printer::from_platform_printer_getters(p));
    }
}
