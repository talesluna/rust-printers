use std::io::Write;
use std::process::{Command, Stdio};

use crate::common::converters::GhostscriptConverterOptions;

pub fn convert(buffer: &[u8], options: &GhostscriptConverterOptions) -> Result<Vec<u8>, String> {
    let output = run(options, "-", Some(buffer.to_vec()))?;

    Ok(output)
}

fn run(
    options: &GhostscriptConverterOptions,
    input: &str,
    stdin: Option<Vec<u8>>,
) -> Result<Vec<u8>, String> {
    let mut command = Command::new(options.command.unwrap_or(
        #[cfg(target_family = "unix")]
        "gs",
        #[cfg(target_family = "windows")]
        "gswin64c.exe",
    ));

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

        let mut child = command
            .spawn()
            .map_err(|error| format!("Start Ghostscript child proccess failed: {error}"))?;

        {
            let stdin = child.stdin.as_mut().unwrap();
            stdin.write_all(&buffer).unwrap();
        }

        child
            .wait_with_output()
            .map_err(|error| format!("Wait Ghostscript child proccess output failed: {error}"))
    } else {
        command
            .output()
            .map_err(|error| format!("Ghostscript proccess failed: {error}"))
    }?;

    if !output.status.success() {
        return Err(format!(
            "Ghostscript exit with code {}",
            output.status.code().unwrap_or(1),
        ));
    }

    Ok(output.stdout)
}
