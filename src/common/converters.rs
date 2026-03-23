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
    pub fn mswinpr2() -> Self {
        Self::from_device("mswinpr2")
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

    /**
     * Returns true if this converter prints directly to the printer (e.g. mswinpr2)
     * bypassing the winspool RAW data path.
     */
    pub fn is_direct_print(&self) -> bool {
        matches!(
            self,
            Converter::Ghostscript(GhostscriptConverterOptions {
                device: Some("mswinpr2"),
                ..
            })
        )
    }

    /**
     * Directly print a file to a printer using Ghostscript mswinpr2 device.
     * This bypasses the winspool RAW path and renders via GDI.
     */
    pub fn direct_print_file(
        &self,
        printer_system_name: &str,
        file_path: &str,
    ) -> Result<u64, PrintersError> {
        match self {
            Converter::Ghostscript(options) if options.device == Some("mswinpr2") => {
                ghostscript::direct_print_file(options, printer_system_name, file_path)
            }
            _ => Err(PrintersError::print_error(
                "direct_print_file is only supported with mswinpr2 device",
            )),
        }
    }

    /**
     * Directly print a byte buffer to a printer using Ghostscript mswinpr2 device.
     * Writes the buffer to a temporary file and prints it.
     */
    pub fn direct_print_buffer(
        &self,
        printer_system_name: &str,
        buffer: &[u8],
    ) -> Result<u64, PrintersError> {
        match self {
            Converter::Ghostscript(options) if options.device == Some("mswinpr2") => {
                ghostscript::direct_print_buffer(options, printer_system_name, buffer)
            }
            _ => Err(PrintersError::print_error(
                "direct_print_buffer is only supported with mswinpr2 device",
            )),
        }
    }
}
