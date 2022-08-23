use std::process::Command;    

use crate::printer;
use crate::process;

/**
 * Get printers on unix systems using lpstat
 */
pub fn get_printers() -> Vec<printer::Printer> {

    let result = process::exec(Command::new("lpstat").arg("-e"));

    if result.is_ok() {

        let out_str = result.unwrap();
        let lines: Vec<&str> = out_str.split_inclusive("\n").collect();
        let mut printers: Vec<printer::Printer> = Vec::with_capacity(lines.len());

        for line in lines {

            let system_name = line.replace("\n", "");
            let name = String::from(system_name.replace("_", " ").trim());

            printers.push(printer::Printer::new(name, system_name, &self::print));

        }

        return printers;

    }
        
    return Vec::with_capacity(0);

}


/**
 * Print on unix systems using lp
 */
pub fn print(printer_system_name: &str, file_path: &str) -> Result<bool, String> {

    let result = process::exec(
        Command::new("lp").arg("-d").arg(printer_system_name).arg(file_path)
    );

    if result.is_err() {
        return Result::Err(result.unwrap_err());
    }

    return Result::Ok(true)

}
