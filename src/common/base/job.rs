use std::{
    fmt::{Debug, Error, Formatter},
    sync::Arc,
    time::SystemTime,
};

use crate::{
    common::{
        converters::Converter,
        traits::platform::{PlatformActions, PlatformPrinterJobGetters},
    },
    impl_display,
};

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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PaperSize {
    A4,
    Letter,
    Legal,
    MM(u32, u32),
    CM(u32, u32),
    MT(u32, u32),
}

#[derive(Clone, Copy, Debug)]
pub enum ColorMode {
    Color,
    Monochrome,
}

#[derive(Clone, Copy, Debug)]
pub enum Orientation {
    Portrait,
    Landscape,
}

#[derive(Clone, Copy, Debug)]
pub enum DuplexMode {
    Simplex,
    DuplexLongEdge,
    DuplexShortEdge,
}

#[derive(Clone, Copy, Debug)]
pub enum PrintQuality {
    Draft,
    Normal,
    High,
}

#[derive(Clone, Debug)]
pub struct PrinterJobOptions<'a> {
    pub name: Option<&'a str>,
    pub scale: Option<u32>,
    pub copies: Option<u32>,
    pub duplex: Option<DuplexMode>,
    pub collate: Option<bool>,
    pub data_type: Option<&'a str>,
    pub paper_size: Option<PaperSize>,
    pub color_mode: Option<ColorMode>,
    pub orientation: Option<Orientation>,
    pub quality: Option<PrintQuality>,
    pub converter: Option<Arc<dyn Converter>>,
}

impl_display!(PaperSize, ColorMode, Orientation, DuplexMode, PrintQuality);

impl PrinterJobOptions<'_> {
    pub fn default() -> Self {
        Self {
            name: None,
            scale: None,
            copies: None,
            duplex: None,
            collate: None,
            data_type: None,
            converter: None,
            paper_size: None,
            color_mode: None,
            orientation: None,
            quality: None,
        }
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: &'static str) -> Self {
        self.name = Some(name);
        self
    }

    pub fn copies(mut self, copies: u32) -> Self {
        assert!(copies >= 1, "copies must be greater than 0");
        self.copies = Some(copies);
        self
    }

    pub fn converter<T: Converter>(mut self, converter: T) -> Self
    where
        T: Converter + 'static,
    {
        self.converter = Some(Arc::new(converter));
        self
    }

    pub fn paper_size(mut self, paper_size: PaperSize) -> Self {
        self.paper_size = Some(paper_size);
        self
    }

    pub fn color_mode(mut self, color_mode: ColorMode) -> Self {
        self.color_mode = Some(color_mode);
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }

    pub fn duplex(mut self, duplex: DuplexMode) -> Self {
        self.duplex = Some(duplex);
        self
    }

    pub fn quality(mut self, quality: PrintQuality) -> Self {
        self.quality = Some(quality);
        self
    }

    pub fn collate(mut self, collate: bool) -> Self {
        self.collate = Some(collate);
        self
    }

    pub fn scale(mut self, scale: u32) -> Self {
        assert!(scale >= 1 && scale <= 100, "scale must between 1 and 100");
        self.scale = Some(scale);
        self
    }

    pub fn data_type(mut self, data_type: &'static str) -> Self {
        self.data_type = Some(data_type);
        self
    }
}
