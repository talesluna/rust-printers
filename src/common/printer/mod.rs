pub mod state;
pub mod job;

use std::fmt::{Debug, Error, Formatter};

use state::PrinterState;
use self::job::PrinterJob;

use super::{traits::platform::{PlatformActions, PlatformPrinterGetters}, utils};

/**
 * Printer is a struct to representation the system printer
 */
pub struct Printer {
    /**
     * Visual reference of system printer name
     */
    pub name: String,

    /**
     * Name of Printer exactly as on system
     */
    pub system_name: String,

    /**
     * Name of the Printer driver
     */
    pub driver_name: String,

    /**
     * Uri of printer (default is empty string)
     */
    pub uri: String,

    /**
     * Name of printer port (default is empty string)
     */
    pub port_name: String,

    /**
     * Name of printer port (default is empty string)
     */
    pub processor: String,

    /**
     * Name of printer port (default is RAW)
     */
    pub data_type: String,

    /**
     * Name of printer port (default is empty string)
     */
    pub description: String,

    /**
     * Location definition of printer (default is empty string)
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
}

impl Debug for Printer {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            fmt,
            "Printer {{
                \r  name: {:?},
                \r  state: {:?},
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
        return Printer {
            name: self.name.clone(),
            state: self.state.clone(),
            uri: self.uri.clone(),
            location: self.location.clone(),
            port_name: self.port_name.clone(),
            is_default: self.is_default.clone(),
            system_name: self.system_name.clone(),
            driver_name: self.driver_name.clone(),
            is_shared: self.is_shared.clone(),
            data_type: self.data_type.clone(),
            description: self.description.clone(),
            processor: self.processor.clone(),
        };
    }
}

impl Printer {
    pub fn from_platform_printer_getters(platform_printer: &dyn PlatformPrinterGetters) -> Printer {
        let printer = Printer {
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
            state: PrinterState::from_platform_state(platform_printer.get_state().as_str()),
        };

        return printer;
    }

    /**
     * Print bytes with self printer instance
     */
    pub fn print(&self, buffer: &[u8], job_name: Option<&str>) -> Result<(), &'static str> {
        #[cfg(target_family = "unix")] {
            let path = utils::file::save_tmp_file(buffer);
            return if path.is_some() {
                let file_path = path.unwrap();
                let result = self.print_file(file_path.to_str().unwrap(), job_name);
                result
            } else {
                Err("Failed to create temp file")
            };
        }

        #[cfg(target_family = "windows")] {
            return crate::Platform::print(self.system_name.as_str(), buffer, job_name);
        }
    }

    /**
     * Print specific file with self printer instance
     */
    pub fn print_file(&self, file_path: &str, job_name: Option<&str>) -> Result<(), &'static str> {

        #[cfg(target_family = "unix")] {
            return crate::Platform::print(self.system_name.as_str(), file_path, job_name);
        }

        #[cfg(target_family = "windows")] {
            let buffer = utils::file::get_file_as_bytes(file_path);
            return if buffer.is_some() {
                crate::Platform::print(self.system_name.as_str(), &buffer.unwrap(), job_name)
            } else {
                Err("failed to read file")
            }
        }
    }
    
    /**
     * 
     */
    pub fn get_active_jobs(&self) -> Vec<PrinterJob> {
        return crate::Platform::get_printer_jobs(self.system_name.as_str(), true);
    }

    /**
     * 
     */
    pub fn get_job_history(&self) -> Vec<PrinterJob> {
        return crate::Platform::get_printer_jobs(self.system_name.as_str(), false);
    }

}