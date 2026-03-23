use std::io::Write;
use std::process::{Command, Stdio};

use crate::common::{base::errors::PrintersError, converters::GhostscriptConverterOptions};

fn gs_command(options: &GhostscriptConverterOptions) -> Command {
    Command::new(match options.command {
        Some(v) => v,
        #[cfg(target_family = "unix")]
        None => "gs",
        #[cfg(target_family = "windows")]
        None => "gswin64c.exe",
    })
}

pub fn direct_print_file(
    options: &GhostscriptConverterOptions,
    printer_system_name: &str,
    file_path: &str,
) -> Result<u64, PrintersError> {
    let output_file = format!("%printer%{printer_system_name}");
    let mut command = gs_command(options);

    command.args([
        "-dBATCH",
        "-dNOPAUSE",
        "-sDEVICE=mswinpr2",
        &format!("-sOutputFile={output_file}"),
    ]);

    if let Some(dpi) = options.dpi {
        command.arg(format!("-r{dpi}"));
    }

    command.arg(file_path);
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    let output = command.output().map_err(PrintersError::print_error)?;

    if output.status.success() {
        Ok(0)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(PrintersError::print_error(format!(
            "Ghostscript mswinpr2 exit with code {}: {stderr}",
            output.status.code().unwrap_or(1)
        )))
    }
}

pub fn direct_print_buffer(
    options: &GhostscriptConverterOptions,
    printer_system_name: &str,
    buffer: &[u8],
) -> Result<u64, PrintersError> {
    let temp_dir = std::env::temp_dir();
    let temp_file = temp_dir.join(format!("printers_mswinpr2_{}.pdf", std::process::id()));
    std::fs::write(&temp_file, buffer).map_err(PrintersError::file_error)?;

    let result = direct_print_file(
        options,
        printer_system_name,
        temp_file.to_str().unwrap_or(""),
    );

    let _ = std::fs::remove_file(&temp_file);
    result
}

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
