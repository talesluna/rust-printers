mod ghostscript;

pub enum GhostscriptConverterDevice {
    Ps2write,
    Png16m,
    TiffG4,
    PngMono,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct GhostscriptConverterOptions {
    pub command: Option<&'static str>,
    pub dpi: Option<u32>,
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

#[derive(Debug, Clone, PartialEq)]
pub enum Converter {
    None,
    Ghostscript(GhostscriptConverterOptions),
}

impl Converter {
    pub fn convert(&self, buffer: &[u8]) -> Result<Vec<u8>, String> {
        match self {
            Converter::Ghostscript(options) => ghostscript::convert(buffer, options),
            Converter::None => Ok(buffer.to_vec()),
        }
    }
}
