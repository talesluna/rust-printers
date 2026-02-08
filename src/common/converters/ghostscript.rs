use crate::common::base::errors::PrintersError;
use std::io::Write;
use std::process::{Command, Stdio};

#[derive(Debug, Clone)]
pub struct GhostscriptConverter {
    dpi: u32,
    device: &'static str,
    command: &'static str,
}

impl GhostscriptConverter {
    pub fn new() -> Self {
        Self {
            dpi: 500,
            device: "ps2write",
            #[cfg(target_family = "unix")]
            command: "gs",
            #[cfg(target_family = "windows")]
            command: "gswin64c.exe",
        }
    }

    pub fn dpi(mut self, dpi: u32) -> Self {
        self.dpi = dpi;
        self
    }

    pub fn command(mut self, command: &'static str) -> Self {
        self.command = command;
        self
    }

    pub fn device(mut self, device: &'static str) -> Self {
        self.device = device;
        self
    }

    pub fn ps2write(mut self) -> Self {
        self.device = "ps2write";
        self
    }

    pub fn png16m(mut self) -> Self {
        self.device = "png16m";
        self
    }

    pub fn jpeg(mut self) -> Self {
        self.device = "jpeg";
        self
    }

    pub fn bmp16m(mut self) -> Self {
        self.device = "bmp16m";
        self
    }

    pub fn pngmono(mut self) -> Self {
        self.device = "pngmono";
        self
    }

    pub fn pdfwrite(mut self) -> Self {
        self.device = "pdfwrite";
        self
    }
}

pub fn convert(buffer: &[u8], options: &GhostscriptConverter) -> Result<Vec<u8>, PrintersError> {
    let output = run(options, "-", Some(buffer.to_vec()))?;
    Ok(output)
}

fn run(
    options: &GhostscriptConverter,
    input: &str,
    stdin: Option<Vec<u8>>,
) -> Result<Vec<u8>, PrintersError> {
    let mut command = Command::new(options.command);

    command.args([
        "-q",
        "-dSAFER",
        "-dBATCH",
        "-dNOPAUSE",
        format!("-sDEVICE={}", options.device).as_str(),
        format!("-r{}", options.dpi).as_str(),
        "-sOutputFile=%stdout",
        input,
    ]);

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
