use crate::common::base::{errors::PrintersError, job::PrinterJobOptions};
use std::fmt::Debug;

mod ghostscript;

pub trait Converter: Debug {
    fn convert(
        &self,
        buffer: &[u8],
        job_options: &PrinterJobOptions,
    ) -> Result<Vec<u8>, PrintersError>;
}

pub struct Converters {}
impl Converters {
    pub fn ghostscript() -> ghostscript::GhostscriptConverter {
        ghostscript::GhostscriptConverter::default()
    }
}

impl Converter for ghostscript::GhostscriptConverter {
    fn convert(
        &self,
        buffer: &[u8],
        job_options: &PrinterJobOptions,
    ) -> Result<Vec<u8>, PrintersError> {
        ghostscript::convert(buffer, job_options, self)
    }
}
