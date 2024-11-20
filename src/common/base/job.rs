use std::{fmt::{Debug, Formatter, Error}, time::SystemTime};

use crate::common::traits::platform::PlatformPrinterJobGetters;

#[derive(Debug, Clone)]
pub enum PrinterJobState {
    PENDING,
    PAUSED,
    PROCCESSING,
    CANCELLED,
    COMPLETED,
    UNKNOWN,
}

pub struct PrinterJob {
    /**
     * Job ID
     */
    pub id: u32,
    /**
     * Visual name/title of job
     */
    pub name: String,
    /**
     * Job Status, indicates how the job is currently
     */
    pub state: PrinterJobState,
    /**
     * Indicates the job file type, ex application/pdf
     */
    pub media_type: String,
    /**
     * Date when job was created
     */
    pub created_at: SystemTime,
    /**
     * Date when job was processed or started printing 
     */
    pub processed_at: Option<SystemTime>,
    /**
     * Date when job was completed
     */
    pub completed_at: Option<SystemTime>,
    /**
     * Name of printer
     */
    pub printer_name: String,
}

impl PrinterJob {

    pub fn from_platform_printer_job_getters(platform_printer_job: &dyn PlatformPrinterJobGetters) -> Self {
        return PrinterJob {
            id: platform_printer_job.get_id(),
            name: platform_printer_job.get_name(),
            state: PrinterJobState::from_platform_state(platform_printer_job.get_state()),
            media_type: platform_printer_job.get_media_type(),
            created_at: platform_printer_job.get_created_at(),
            processed_at: platform_printer_job.get_processed_at(),
            completed_at: platform_printer_job.get_completed_at(),
            printer_name: platform_printer_job.get_printer(),
        }
    }

}

impl Debug for PrinterJob {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            fmt,
            "PrinterJob {{
                \r  id: {:?},
                \r  name: {:?},
                \r  state: {:?},
                \r  media_type: {:?},
                \r  created_at: {:?},
                \r  processed_at: {:?},
                \r  completed_at: {:?},
                \r  printer_name: {:?},
            \r}}",
            self.id,
            self.name,
            self.state,
            self.media_type,
            self.created_at,
            self.processed_at,
            self.completed_at,
            self.printer_name,
        )
    }
}

impl PrinterJobState {
    
    pub fn from_platform_state(_platform_state: u32) -> Self {
        #[cfg(target_family = "unix")] {
            if _platform_state == 3 {
                return PrinterJobState::PENDING;
            }
            
            if _platform_state == 4 || _platform_state == 6 {
                return PrinterJobState::PAUSED;
            }
    
            if _platform_state == 5 {
                return PrinterJobState::PROCCESSING;
            }
    
            if _platform_state == 7 || _platform_state == 8 {
                return PrinterJobState::CANCELLED;
            }
    
            if _platform_state == 9 {
                return PrinterJobState::COMPLETED;
            }

            return PrinterJobState::UNKNOWN;
        }

        #[cfg(target_family = "windows")]
        return PrinterJobState::UNKNOWN;

    }
}