use crate::common::base::errors::PrintersError;
use std::fmt::Debug;

mod ghostscript;

pub trait Converter: Debug {
    fn convert(&self, buffer: &[u8]) -> Result<Vec<u8>, PrintersError>;
}

pub struct Converters {}
impl Converters {
    pub fn ghostscript() -> ghostscript::GhostscriptConverter {
        ghostscript::GhostscriptConverter::new()
    }
}

impl Converter for ghostscript::GhostscriptConverter {
    fn convert(&self, buffer: &[u8]) -> Result<Vec<u8>, PrintersError> {
        ghostscript::convert(buffer, self)
    }
}
