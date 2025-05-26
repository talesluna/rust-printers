use std::{
    fmt::{Debug, Error, Formatter},
    time::SystemTime,
};

use crate::common::traits::platform::{PlatformActions, PlatformPrinterJobGetters};

#[derive(Debug, Clone, PartialEq)]
pub enum PrinterJobState {
    PENDING,
    PAUSED,
    PROCESSING,
    CANCELLED,
    COMPLETED,
    UNKNOWN,
}

pub struct PrinterJob {
    /**
     * Job ID
     */
    pub id: u64,
    /**
     * Visual name/title of a job
     */
    pub name: String,
    /**
     * Job Status indicates how the job is currently
     */
    pub state: PrinterJobState,
    /**
     * Indicates the job file type, ex application/pdf
     */
    pub media_type: String,
    /**
     * Date when a job was created
     */
    pub created_at: SystemTime,
    /**
     * Date when a job was processed or started printing
     */
    pub processed_at: Option<SystemTime>,
    /**
     * Date when a job was completed
     */
    pub completed_at: Option<SystemTime>,
    /**
     * Name of printer
     */
    pub printer_name: String,
}

impl PrinterJob {
    pub(crate) fn from_platform_printer_job_getters(
        platform_printer_job: &dyn PlatformPrinterJobGetters,
    ) -> Self {
        PrinterJob {
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
    pub(crate) fn from_platform_state(platform_state: u64) -> Self {
        crate::Platform::parse_printer_job_state(platform_state)
    }
}

pub struct PrinterJobOptions<'a> {
    pub name: Option<&'a str>,
    pub raw_properties: &'a [(&'a str, &'a str)],
}

impl PrinterJobOptions<'_> {
    pub fn none() -> Self {
        return PrinterJobOptions {
            name: None,
            raw_properties: &[],
        };
    }
}
