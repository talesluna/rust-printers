use std::process::Command;    

use crate::printer;

/**
 * Get printers on unix systems using lpstat
 */
pub fn get_printers() -> Vec<printer::Printer> {
    let out = Command::new("lpstat").arg("-e").output().unwrap();
    if out.status.success() {
        unsafe {

            let out_str = String::from_utf8_unchecked(out.stdout);
            let lines: Vec<&str> = out_str.split_inclusive("\n").collect();
            let mut printers: Vec<printer::Printer> = Vec::with_capacity(lines.len());

            for line in lines {

                let system_name = line.replace("\n", "");
                let name = String::from(system_name.replace("_", " ").trim());

                let executor = Box::new(|printer_system_name: &str, file_path: &str| {
                    return self::print(printer_system_name, file_path)
                });

                printers.push(printer::Printer::new(name, system_name, executor));

            }

            return printers;

        };
    } else {
        return Vec::with_capacity(0);
    }
}


/**
 * Print on unix systems using lp
 */
pub fn print(printer_system_name: &str, file_path: &str) -> Result<bool, String> {
    let process = Command::new("lp")
        .arg("-d")
        .arg(printer_system_name)
        .arg(file_path)
        .output();

    if process.is_err() {
        return Result::Err(process.unwrap_err().to_string());
    }

    return Result::Ok(true);

}
