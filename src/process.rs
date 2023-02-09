use std::process::Command;

/**
 * Execute an command and return result of stderr (Err) or stdout (Ok)
 */
pub fn exec(command: &mut Command) -> Result<String, String> {
    let out = command.output();

    if let Err(error) = out {
        return Result::Err(error.to_string());
    }

    Result::Ok(String::from_utf8(out.unwrap().stdout).unwrap())
}
