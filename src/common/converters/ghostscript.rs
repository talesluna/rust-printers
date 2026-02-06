use std::io::Write;
use std::process::{Command, Stdio};

use crate::common::{base::errors::PrintersError, converters::GhostscriptConverterOptions};

pub fn convert(
    buffer: &[u8],
    options: &GhostscriptConverterOptions,
) -> Result<Vec<u8>, PrintersError> {
    let output = run(options, "-", Some(buffer.to_vec()))?;
    Ok(output)
}

fn run(
    options: &GhostscriptConverterOptions,
    input: &str,
    stdin: Option<Vec<u8>>,
) -> Result<Vec<u8>, PrintersError> {
    let mut command = Command::new(match options.command {
        Some(v) => v,
        #[cfg(target_family = "unix")]
        None => "gs",
        #[cfg(target_family = "windows")]
        None => "gswin64c.exe",
    });

    command.args([
        "-q",
        "-dSAFER",
        "-dBATCH",
        "-dNOPAUSE",
        format!("-sDEVICE={}", options.device.unwrap_or("png16m")).as_str(),
        format!("-r{}", options.dpi.unwrap_or(500)).as_str(),
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
