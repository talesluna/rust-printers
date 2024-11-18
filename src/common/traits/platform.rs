use std::time::SystemTime;

pub trait PlatformPrinterGetters {
    fn get_name(&self) -> String;
    fn get_is_default(&self) -> bool;
    fn get_system_name(&self) -> String;
    fn get_marker_and_model(&self) -> String;
    fn get_is_shared(&self) -> bool;
    fn get_uri(&self) -> String;
    fn get_location(&self) -> String;
    fn get_state(&self) -> String;
    fn get_port_name(&self) -> String;
    fn get_processor(&self) -> String;
    fn get_description(&self) -> String;
    fn get_data_type(&self) -> String;
}

pub trait PlatformPrinterJobGetters {
    fn get_id(&self) -> u32;
    fn get_name(&self) -> String;
    fn get_state(&self) -> u32;
    fn get_printer(&self) -> String;
    fn get_media_type(&self) -> String;
    fn get_created_at(&self) -> SystemTime;
    fn get_processed_at(&self) -> Option<SystemTime>;
    fn get_completed_at(&self) -> Option<SystemTime>;
}

pub trait PlatformActions {
    fn get_printers() -> Vec<crate::common::printer::Printer>;
    
    #[cfg(target_family = "windows")]
    fn print(printer_system_name: &str, buffer: &[u8], job_name: Option<&str>) -> Result<(), &'static str>;
    
    #[cfg(target_family = "unix")]
    fn print(printer_system_name: &str, file_path: &str, job_name: Option<&str>) -> Result<(), &'static str>;

    fn get_printer_jobs(printer_name: &str, active_only: bool) -> Vec<crate::common::printer::job::PrinterJob>;
    fn get_default_printer() -> Option<crate::common::printer::Printer>;
    fn get_printer_by_name(printer_name: &str) -> Option<crate::common::printer::Printer>;
}