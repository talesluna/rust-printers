use std::{
    fmt::{Debug, Error, Formatter},
    sync::Arc,
    time::SystemTime,
};

use crate::{
    Platform,
    common::{
        base::errors::PrintersError,
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
    Custom(i32, i32, &'static str, i32)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColorMode {
    Color,
    Monochrome,
}

#[derive(Clone, Copy, Debug, PartialEq)]
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

#[derive(Clone, Debug, Default)]
pub struct PrinterJobOptions {
    pub name: Option<String>,
    pub scale: Option<i16>,
    pub copies: Option<i16>,
    pub duplex: Option<DuplexMode>,
    pub collate: Option<bool>,
    pub quality: Option<PrintQuality>,
    pub data_type: Option<String>,
    pub paper_size: Option<PaperSize>,
    pub color_mode: Option<ColorMode>,
    pub orientation: Option<Orientation>,
    pub converter: Option<Arc<dyn Converter>>,
    printer_name: Option<String>,
}

impl_display!(PaperSize, ColorMode, Orientation, DuplexMode, PrintQuality);

impl PrinterJobOptions {
    pub fn from_printer(printer_name: &str) -> Self {
        Self {
            printer_name: Some(printer_name.into()),
            ..Default::default()
        }
    }

    pub fn name(mut self, name: &'static str) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn copies(mut self, copies: i16) -> Self {
        assert!(copies >= 1, "copies must be greater than 0");
        self.copies = Some(copies);
        self
    }

    pub fn converter<T>(mut self, converter: T) -> Self
    where
        T: Converter + 'static,
    {
        self.converter = Some(Arc::new(converter));
        self
    }

    pub fn paper_size(mut self, paper_size: PaperSize) -> Self {
        assert!(self.paper_size.is_none(), "paper_size duplicated");
        self.paper_size = Some(paper_size);
        self
    }

    pub fn paper_size_mm(mut self, w: i32, h: i32) -> Self {
        assert!(self.paper_size.is_none(), "paper_size duplicated");
        self.paper_size = Some(PaperSize::Custom(w, h, "mm", 1));
        self
    }

    pub fn paper_size_cm(mut self, w: i32, h: i32) -> Self {
        assert!(self.paper_size.is_none(), "paper_size duplicated");
        self.paper_size = Some(PaperSize::Custom(w, h, "cm", 100));
        self
    }

    pub fn paper_size_mt(mut self, w: i32, h: i32) -> Self {
        assert!(self.paper_size.is_none(), "paper_size duplicated");
        self.paper_size = Some(PaperSize::Custom(w, h, "mt", 1000));
        self
    }

    pub fn color(mut self) -> Self {
        assert!(self.color_mode.is_none(), "color_mode duplicated");
        self.color_mode = Some(ColorMode::Color);
        self
    }

    pub fn monochrome(mut self) -> Self {
        assert!(self.color_mode.is_none(), "color_mode duplicated");
        self.color_mode = Some(ColorMode::Monochrome);
        self
    }

    pub fn landscape(mut self) -> Self {
        assert!(self.orientation.is_none(), "orientation duplicated");
        self.orientation = Some(Orientation::Landscape);
        self
    }

    pub fn portrait(mut self) -> Self {
        assert!(self.orientation.is_none(), "orientation duplicated");
        self.orientation = Some(Orientation::Portrait);
        self
    }

    pub fn simplex(mut self) -> Self {
        assert!(self.duplex.is_none(), "duplex duplicated");
        self.duplex = Some(DuplexMode::Simplex);
        self
    }

    pub fn duplex_long(mut self) -> Self {
        assert!(self.duplex.is_none(), "duplex duplicated");
        self.duplex = Some(DuplexMode::DuplexLongEdge);
        self
    }

    pub fn duplex_short(mut self) -> Self {
        assert!(self.duplex.is_none(), "duplex duplicated");
        self.duplex = Some(DuplexMode::DuplexShortEdge);
        self
    }

    pub fn quality_normal(mut self) -> Self {
        assert!(self.quality.is_none(), "quality duplicated");
        self.quality = Some(PrintQuality::Normal);
        self
    }

    pub fn quality_draft(mut self) -> Self {
        assert!(self.quality.is_none(), "quality duplicated");
        self.quality = Some(PrintQuality::Draft);
        self
    }

    pub fn quality_high(mut self) -> Self {
        assert!(self.quality.is_none(), "quality duplicated");
        self.quality = Some(PrintQuality::High);
        self
    }

    pub fn collate(mut self, collate: bool) -> Self {
        self.collate = Some(collate);
        self
    }

    pub fn scale(mut self, scale: i16) -> Self {
        assert!((1..=100).contains(&scale), "scale must between 1 and 100");
        self.scale = Some(scale);
        self
    }

    pub fn data_type(mut self, data_type: &'static str) -> Self {
        self.data_type = Some(data_type.into());
        self
    }

    pub fn print(self, buffer: &[u8]) -> Result<u64, PrintersError> {
        if let Some(printer_system_name) = &self.printer_name {
            Platform::print(printer_system_name, buffer, &self)
        } else {
            Err(PrintersError::print_error("unknown printer"))
        }
    }

    pub fn print_file(self, file_path: &str) -> Result<u64, PrintersError> {
        if let Some(printer_system_name) = &self.printer_name {
            Platform::print_file(printer_system_name, file_path, &self)
        } else {
            Err(PrintersError::print_error("unknown printer"))
        }
    }
}
