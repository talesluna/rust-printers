use std::process::Command;    

use crate::printer;
use crate::printer::PrinterOption;
use crate::process;

/**
 * Get printers on unix systems using lpstat
 */
pub fn get_printers() -> Vec<printer::Printer> {

    let result = process::exec(Command::new("lpstat").arg("-e"));

    if let Ok(out_str) = result {

        let lines: Vec<&str> = out_str.split_inclusive('\n').collect();
        let mut printers: Vec<printer::Printer> = Vec::with_capacity(lines.len());

        for line in lines {

            let system_name = line.replace('\n', "");
            let name = String::from(system_name.replace('_', " ").trim());
            let options = Vec::new();
            printers.push(printer::Printer::new(name, system_name, options, &self::print));

        }

        return printers;

    }
        
    Vec::with_capacity(0)

}


/**
 * Print on unix systems using lp
 */
pub fn print(printer_system_name: &str, file_path: &str, options: &[PrinterOption]) -> Result<bool, String> {

    let mut cmd = Command::new("lp");
    cmd.arg("-d").arg(printer_system_name).arg(file_path);

    for option in options.iter(){
        cmd.arg("-o").arg(option.to_str());
    }
    let result = process::exec(&mut cmd);

    if let Err(error) = result {
        return Result::Err(error);
    }

    Result::Ok(true)
}
