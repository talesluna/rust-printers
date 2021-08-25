use uuid::Uuid;
use std::process::Command;

/**
 * Get printers on windows using wmic
 */
pub fn get_printers() -> Vec<super::Printer> {

    let out = Command::new("wmic")
        .arg("printer")
        .arg("get")
        .arg("DriverName, Name")
        .output()
        .unwrap();

    if out.status.success() {
        unsafe {

            let out_str = String::from_utf8_unchecked(out.stdout);
            let mut lines: Vec<&str> = out_str.split_inclusive("\n").collect();
            lines.remove(0);

            let mut printers: Vec<super::Printer> = Vec::with_capacity(lines.len());
            for line in lines {
                let printer_data: Vec<&str> = line.split_ascii_whitespace().collect();
                printers.push(super::Printer {
                    id: Uuid::new_v5(&Uuid::NAMESPACE_DNS, printer_data[0].as_bytes()).to_string(),
                    name: String::from(printer_data[1]),
                    system_name: String::from(printer_data[0]),
                });
            }

            return printers;
        }
    }

    return Vec::with_capacity(0);

}


/**
 * Print on windows using lpr
 */
pub fn print(printer_system_name: &String, file_path: &std::path::PathBuf) -> bool {

    let process = Command::new("lpr")
        .arg("-P")
        .arg(printer_system_name)
        .arg(file_path)
        .output()
        .unwrap();

    return process.status.success();

}