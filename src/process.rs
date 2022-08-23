use std::process::Command;

/**
 * Execute an command and return result of stderr (Err) or stdout (Ok)
 */
pub fn exec(command: &mut Command) -> Result<String, String> {
    let out = command.output();

    if out.is_err() {
        return Result::Err(out.unwrap_err().to_string());
    }

    return Result::Ok(String::from_utf8(out.unwrap().stdout).unwrap());
}
