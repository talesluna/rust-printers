use crate::common::base::errors::PrintersError;
use crate::common::base::job::{ColorMode, PrinterJobOptions};
use std::io::Write;
use std::process::{Command, Stdio};

#[derive(Debug, Clone)]
pub struct GhostscriptConverter {
    device: &'static str,
    command: &'static str,
    apply_job_options: bool,
    device_gray: Option<&'static str>,
}

impl Default for GhostscriptConverter {
    fn default() -> Self {
        #[cfg(target_family = "windows")] {
            return Self {
                device: "ps2write",
                command: "gswin64c.exe",
                device_gray: None,
                // WINDOWS TRUE BY DEFAULT - AUTOMATE OPTIONS WHEN GHOSTSCRIPT AVAILABLE
                apply_job_options: true,
            };
        }

        Self {
            device: "pdfwrite",
            command: "gs",
            device_gray: None,
            apply_job_options: false,
        }
    }
}

impl GhostscriptConverter {
    pub fn command(mut self, command: &'static str) -> Self {
        self.command = command;
        self
    }

    pub fn device(mut self, device: &'static str) -> Self {
        self.device = device;
        self
    }

    pub fn apply_job_options(mut self, apply: bool) -> Self {
        self.apply_job_options = apply;
        self
    }

    pub fn to_postscript(mut self) -> Self {
        self.device = "ps2write";
        self
    }

    pub fn to_png(mut self) -> Self {
        self.device = "png16m";
        self.device_gray = Some("pnggray");
        self
    }

    pub fn to_jpeg(mut self) -> Self {
        self.device = "jpeg";
        self
    }

    pub fn to_bitmap(mut self) -> Self {
        self.device = "bmp16m";
        self.device_gray = Some("bmpgray");
        self
    }

    pub fn to_pdf(mut self) -> Self {
        self.device = "pdfwrite";
        self
    }
}

pub fn convert(
    buffer: &[u8],
    job_options: &PrinterJobOptions,
    options: &GhostscriptConverter,
) -> Result<Vec<u8>, PrintersError> {
    let output = run(options, "-", Some(buffer.to_vec()), job_options)?;
    Ok(output)
}

fn run(
    options: &GhostscriptConverter,
    input: &str,
    stdin: Option<Vec<u8>>,
    job_options: &PrinterJobOptions,
) -> Result<Vec<u8>, PrintersError> {

    let device = if options.apply_job_options
        && job_options.color_mode == Some(ColorMode::Gray)
        && let Some(device_gray) = options.device_gray
    {
        device_gray
    } else {
        options.device
    };

    let mut command = Command::new(options.command);

    command.args([
        "-q",
        "-dSAFER",
        "-dBATCH",
        "-dNOPAUSE",
        format!("-sDEVICE={}", device).as_str(),
        "-sOutputFile=%stdout",
    ]);

    if options.apply_job_options {
        command.args(job_options_into_gs_options(job_options));
    }

    command.args(["-f", input]);

    println!("{}", format!("{:?}", command).replace("\"", ""));

    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    let output = if let Some(buffer) = stdin {
        command.stdin(Stdio::piped());

        let mut child = command.spawn().map_err(PrintersError::converter_error)?;

        if let Some(stdin) = child.stdin.as_mut() {
            stdin
                .write_all(&buffer)
                .map_err(PrintersError::converter_error)?;
        }

        child
            .wait_with_output()
            .map_err(PrintersError::converter_error)
    } else {
        command.output().map_err(PrintersError::converter_error)
    }?;

    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(PrintersError::converter_error(format!(
            "Ghostscript exit with code {}",
            output.status.code().unwrap_or(1)
        )))
    }
}

pub fn job_options_into_gs_options(job_options: &PrinterJobOptions) -> Vec<String> {
    use crate::common::base::job::{ColorMode, DuplexMode, Orientation, PaperSize, PrintQuality};

    let mut gs_options: Vec<String> = Vec::new();

    let landscape = job_options.orientation == Some(Orientation::Landscape);

    if job_options.color_mode == Some(ColorMode::Gray) {
        gs_options.push("-dProcessColorModel=/DeviceGray".into());
        gs_options.push("-dColorConversionStrategy=/Gray".into());
    }

    if let Some(quality) = job_options.quality {
        gs_options.push(format!(
            "-r{}",
            match quality {
                PrintQuality::High => 800,
                PrintQuality::Draft => 200,
                PrintQuality::Normal => 400,
            }
        ));
    }

    if let Some(paper_size) = job_options.paper_size {
        let points = match paper_size {
            PaperSize::Custom(w, h, _, multi) => {
                let w = (((w * multi) as f64 * 72.0) / 25.4).round() as i32;
                let h = (((h * multi) as f64 * 72.0) / 25.4).round() as i32;
                if landscape { (h, w) } else { (w, h) }
            }
            _ => (0, 0),
        };

        if points.0 == 0 && points.1 == 0 {
            gs_options.push(format!(
                "-sPAPERSIZE={}{}",
                paper_size.to_string().to_lowercase(),
                if landscape { "rotated" } else { "" }
            ));
        } else {
            if points.0 > 0 {
                gs_options.push(format!("-dDEVICEWIDTHPOINTS={}", points.0));
            }
            if points.1 > 0 {
                gs_options.push(format!("-dDEVICEHEIGHTPOINTS={}", points.1));
            }
        }

    }

    if let Some(collate) = job_options.collate {
        gs_options.push(format!("-dCollate={collate}"));
    }

    if let Some(duplex_mode) = job_options.duplex {
        match duplex_mode {
            DuplexMode::Simplex => gs_options.push("-dDuplex=false".into()),
            duplex => {
                gs_options.push("-dDuplex=true".into());
                gs_options.push(format!(
                    "-dTumble={}",
                    duplex == DuplexMode::DuplexShortEdge
                ));
            }
        };
    }

    if landscape {
        gs_options.push("-c \"<</Orientation 3>> setpagedevice\"".into());
    }

    gs_options
}
