use std::str;
use std::process::Command;    


use crate::printer;

/**
 * Get printers on unix systems using lpstat
 */
pub fn get_printers() -> Vec<printer::Printer> {

    let command = Command::new("lpstat")
        .arg("-e")
        .output()
        .unwrap();

    if command.status.success() {

        let out_str = str::from_utf8(&command.stdout).unwrap();
        let lines: Vec<&str> = out_str.split_inclusive("\n").collect();
        let mut printers: Vec<printer::Printer> = Vec::with_capacity(lines.len());

        for line in lines {

            let system_name = line.replace("\n", "");
            let name = String::from(system_name.replace("_", " ").trim());

            printers.push(printer::Printer::new(
                name,
                system_name,
                "".to_string(),
                &self::print)
            );

        }

        return printers;

    }
        
    return Vec::with_capacity(0);

}


/**
 * Print on unix systems using lp
 */
pub fn print(printer_system_name: &str, file_path: &str) -> Result<bool, String> {

    let command = Command::new("lp")
        .arg("-d")
        .arg(printer_system_name).arg(file_path)
        .output()
        .unwrap();

    if command.status.success() {
        return Result::Ok(true)
    }
    
    return Result::Err(str::from_utf8(&command.stderr).unwrap().to_string());

}
