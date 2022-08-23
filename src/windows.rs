use std::process::Command;

use crate::printer;
use crate::process;

/**
 * Get printers on windows using wmic
 */
pub fn get_printers() -> Vec<printer::Printer> {

    let result = process::exec(
        Command::new("wmic").arg("printer").arg("get").arg("DriverName, Name")
    );

    if result.is_ok() {

        let out_str = result.unwrap();
        let mut lines: Vec<&str> = out_str.split_inclusive("\n").collect();
        lines.remove(0);

        let mut printers: Vec<printer::Printer> = Vec::with_capacity(lines.len());

        for line in lines {
            
            let printer_data: Vec<&str> = line.split_ascii_whitespace().collect();

            let name = String::from(printer_data[1]);
            let system_name = String::from(printer_data[0]);

            printers.push(printer::Printer::new(name, system_name,  &self::print));

        }

        return printers;

    }

    return Vec::with_capacity(0);

}


/**
 * Print on windows using lpr
 */
pub fn print(printer_system_name: &str, file_path: &str) -> Result<bool, String> {

    let process = process::exec(
        Command::new("lpr").arg("-S 127.0.0.1").arg("-P").arg(printer_system_name).arg(file_path)
    );

    if process.is_err() {
        return Result::Err(process.unwrap_err());
    }

    return Result::Ok(true);

}