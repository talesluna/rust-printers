use std::process::Command;

use crate::printer;
use crate::printer::PrinterOption;
use crate::process;

/**
 * Get printers on windows using wmic
 */
pub fn get_printers() -> Vec<printer::Printer> {

    let result = process::exec(
        Command::new("wmic").arg("printer").arg("get").arg("DriverName,Name")
    );

    if let Ok(out_str) = result {

        let mut lines: Vec<&str> = out_str.split_inclusive('\n').collect();
        lines.remove(0);

        let mut printers: Vec<printer::Printer> = Vec::with_capacity(lines.len());

        for line in lines {
            
            let printer_data: Vec<&str> = line.split_ascii_whitespace().collect();

            let name = String::from(printer_data[1]);
            let system_name = String::from(printer_data[0]);
            let options = Vec::new();

            printers.push(printer::Printer::new(name, system_name,  options, &self::print));

        }

        return printers;

    }

    Vec::with_capacity(0)

}


/**
 * Print on windows using lpr
 */
pub fn print(printer_system_name: &str, file_path: &str, options: &[PrinterOption]) -> Result<bool, String> {

    let mut cmd = Command::new("lpr");
    cmd.arg("-S 127.0.0.1").arg("-P").arg(printer_system_name).arg(file_path);
    
    for option in options.iter(){
        cmd.arg("-o").arg(option.to_str());
    }
    
    let process = process::exec(&mut cmd);

    if let Err(err) = process {
        return Result::Err(err);
    }

    Result::Ok(true)

}