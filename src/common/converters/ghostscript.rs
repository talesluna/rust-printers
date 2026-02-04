use std::io::Write;
use std::process::{Command, Stdio};

use crate::common::converters::GhostscriptConverterOptions;

pub fn file_to_file(path: &str, options: &GhostscriptConverterOptions) -> Result<String, String> {

    // TODO: save a tmp file here
    let converted_path = format!("{path}.converted");
    run(options, converted_path.as_str(), path, None, None, None)?;

    Ok(converted_path)
}

pub fn vec_to_vec(buffer: &[u8], options: &GhostscriptConverterOptions) -> Result<Vec<u8>, String> {
    let output = run(
        options,
        "%stdout",
        "-",
        Some(buffer.to_vec()),
        Some(Stdio::piped()),
        Some(Stdio::piped()),
    )?;

    Ok(output)
}

fn run(
    options: &GhostscriptConverterOptions,
    output: &str,
    input: &str,
    stdin: Option<Vec<u8>>,
    stdout: Option<Stdio>,
    stderr: Option<Stdio>,
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
        format!("-sOutputFile={output}").as_str(),
        input,
    ]);

    if let Some(stdout) = stdout {
        command.stdout(stdout);
    }

    if let Some(stderr) = stderr {
        command.stderr(stderr);
    }

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
