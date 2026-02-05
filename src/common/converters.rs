use crate::common::base::errors::PrintersError;

mod ghostscript;

/**
 * Known ghostscript devices or custom
 */
pub enum GhostscriptConverterDevice {
    Ps2write,
    Png16m,
    TiffG4,
    PngMono,
    Custom(String),
}

/**
 * Ghostscript converter options
 */
#[derive(Debug, Clone, PartialEq)]
pub struct GhostscriptConverterOptions {
    /**
     * The path of ghostscript executable/bin 
     */
    pub command: Option<&'static str>,
    /**
     * The output resolution
     */
    pub dpi: Option<u32>,
    /**
     * The output type
     */
    pub device: Option<&'static str>,
}

impl GhostscriptConverterOptions {
    pub fn ps2write() -> Self {
        Self::from_device("ps2write")
    }
    pub fn png16m() -> Self {
        Self::from_device("png16m")
    }
    pub fn tiffg4() -> Self {
        Self::from_device("tiffg4")
    }
    pub fn pngmono() -> Self {
        Self::from_device("pngmono")
    }
    pub fn from_device(device: &'static str) -> Self {
        Self {
            dpi: None,
            device: Some(device),
            command: None,
        }
    }
}

/**
 * Available converters and their options
 */
#[derive(Debug, Clone, PartialEq)]
pub enum Converter {
    None,
    Ghostscript(GhostscriptConverterOptions),
}

impl Converter {
    /**
     * Converts/raster the contents of a byte array according to the defined converter, into a new byte array.
     * If "None" converter is defined, return the same byte array.
     */
    pub fn convert(&self, buffer: &[u8]) -> Result<Vec<u8>, PrintersError> {
        match self {
            Converter::Ghostscript(options) => ghostscript::convert(buffer, options),
            Converter::None => Ok(buffer.to_vec()),
        }
    }
}
