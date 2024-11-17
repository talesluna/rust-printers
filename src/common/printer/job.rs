use std::{fmt::{Debug, Formatter, Error}, time::SystemTime};

use crate::common::traits::platform::PlatformPrinterJobGetters;
pub struct PrinterJob {
    pub id: u32,
    pub name: String,
    pub state: u32,
    pub media_type: String,
    pub created_at: SystemTime,
    pub processed_at: Option<SystemTime>,
    pub completed_at: Option<SystemTime>,
    pub printer_name: String,
}

impl PrinterJob {

    pub fn from_platform_printer_job_getters(platform_printer_job: &dyn PlatformPrinterJobGetters) -> Self {
        return PrinterJob {
            id: platform_printer_job.get_id(),
            name: platform_printer_job.get_name(),
            state: platform_printer_job.get_state(),
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
            "Printer {{
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