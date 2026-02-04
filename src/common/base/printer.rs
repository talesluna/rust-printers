use std::fmt::{Debug, Error, Formatter};

use super::job::{PrinterJob, PrinterJobOptions};
use crate::common::{
    base::job::PrinterJobState,
    traits::platform::{PlatformActions, PlatformPrinterGetters},
};

#[derive(Debug, Clone, PartialEq)]
pub enum PrinterState {
    READY,
    OFFLINE,
    PAUSED,
    PRINTING,
    UNKNOWN,
}

/**
 * Printer is a struct to representation the system printer
 */
pub struct Printer {
    /**
     * Visual reference of system printer name
     */
    pub name: String,

    /**
     * Name of Printer exactly as on a system
     */
    pub system_name: String,

    /**
     * Name of the Printer driver
     */
    pub driver_name: String,

    /**
     * Uri of printer (default is an empty string)
     */
    pub uri: String,

    /**
     * Name of printer port (default is an empty string)
     */
    pub port_name: String,

    /**
     * Name of printer port (default is an empty string)
     */
    pub processor: String,

    /**
     * Name of printer port (default is RAW)
     */
    pub data_type: String,

    /**
     * Name of printer port (default is an empty string)
     */
    pub description: String,

    /**
     * Location definition of printer (default is an empty string)
     */
    pub location: String,

    /**
     * Definition if the printer is the default printer
     */
    pub is_default: bool,

    /**
     * Definition if the printer is shared
     */
    pub is_shared: bool,

    /**
     * The state of the printer
     */
    pub state: PrinterState,

    /**
     * The state reasons of the printer
     */
    pub state_reasons: Vec<String>,
}

impl Debug for Printer {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            fmt,
            "Printer {{
                \r  name: {:?},
                \r  state: {:?},
                \r  state_reasons: {:?},
                \r  system_name: {:?},
                \r  is_default: {:?},
                \r  uri: {:?},
                \r  port_name: {:?},
                \r  is_shared: {:?},
                \r  location: {:?},
                \r  driver_name: {:?}
                \r  processor: {:?}
                \r  data_type: {:?}
                \r  description: {:?}
            \r}}",
            self.name,
            self.state,
            self.state_reasons,
            self.system_name,
            self.is_default,
            self.uri,
            self.port_name,
            self.is_shared,
            self.location,
            self.driver_name,
            self.processor,
            self.data_type,
            self.description,
        )
    }
}

impl Clone for Printer {
    fn clone(&self) -> Printer {
        Printer {
            name: self.name.clone(),
            state: self.state.clone(),
            state_reasons: self.state_reasons.clone(),
            uri: self.uri.clone(),
            location: self.location.clone(),
            port_name: self.port_name.clone(),
            is_default: self.is_default,
            system_name: self.system_name.clone(),
            driver_name: self.driver_name.clone(),
            is_shared: self.is_shared,
            data_type: self.data_type.clone(),
            description: self.description.clone(),
            processor: self.processor.clone(),
        }
    }
}

impl Printer {
    pub(crate) fn from_platform_printer_getters(
        platform_printer: &dyn PlatformPrinterGetters,
    ) -> Printer {
        let mut state_reasons = platform_printer.get_state_reasons();

        if state_reasons.is_empty() {
            state_reasons.push("none".to_string());
        }

        Printer {
            name: platform_printer.get_name(),
            system_name: platform_printer.get_system_name(),
            driver_name: platform_printer.get_marker_and_model(),
            location: platform_printer.get_location(),
            uri: platform_printer.get_uri(),
            port_name: platform_printer.get_port_name(),
            is_default: platform_printer.get_is_default(),
            is_shared: platform_printer.get_is_shared(),
            data_type: platform_printer.get_data_type(),
            processor: platform_printer.get_processor(),
            description: platform_printer.get_description(),
            state: PrinterState::from_platform_state(
                platform_printer.get_state(),
                state_reasons.join(",").as_str(),
            ),
            state_reasons,
        }
    }

    /**
     * Print bytes
     */
    pub fn print(&self, buffer: &[u8], options: PrinterJobOptions) -> Result<u64, String> {
        crate::Platform::print(self.system_name.as_str(), buffer, options)
    }

    /**
     * Print file
     */
    pub fn print_file(&self, file_path: &str, options: PrinterJobOptions) -> Result<u64, String> {
        crate::Platform::print_file(self.system_name.as_str(), file_path, options)
    }

    /**
     * Return active jobs
     */
    pub fn get_active_jobs(&self) -> Vec<PrinterJob> {
        crate::Platform::get_printer_jobs(self.system_name.as_str(), true)
    }

    /**
     * Return historic jobs
     */
    pub fn get_job_history(&self) -> Vec<PrinterJob> {
        crate::Platform::get_printer_jobs(self.system_name.as_str(), false)
    }

    /**
     * Pause an printer job
     */
    pub fn pause_job(&self, job_id: u64) -> Result<(), String> {
        crate::Platform::set_job_state(&self.system_name, job_id, PrinterJobState::PAUSED)
    }

    /**
     * Resume an paused printer job
     */
    pub fn resume_job(&self, job_id: u64) -> Result<(), String> {
        crate::Platform::set_job_state(&self.system_name, job_id, PrinterJobState::PROCESSING)
    }

    /**
     * restart an printer job
     */
    pub fn restart_job(&self, job_id: u64) -> Result<(), String> {
        crate::Platform::set_job_state(&self.system_name, job_id, PrinterJobState::PENDING)
    }

    /**
     * Cancel an printer job
     */
    pub fn cancel_job(&self, job_id: u64) -> Result<(), String> {
        crate::Platform::set_job_state(&self.system_name, job_id, PrinterJobState::CANCELLED)
    }
}

impl PrinterState {
    pub(crate) fn from_platform_state(platform_state: u64, state_reasons: &str) -> Self {
        crate::Platform::parse_printer_state(platform_state, state_reasons)
    }
}
