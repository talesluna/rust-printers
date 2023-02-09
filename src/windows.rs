use std::process::Command;
use std::str;

use crate::printer;

/**
 * Get printers on windows using wmic
 */
pub fn get_printers() -> Vec<printer::Printer> {
    let command = Command::new("powershell")
        .arg("-Command")
        .arg("Get-Printer | Format-List Name,DriverName")
        .output()
        .unwrap();

    if command.status.success() {
        let out_str = str::from_utf8(&command.stdout).unwrap();
        let lines: Vec<Vec<&str>> = out_str
            .trim()
            .split("\r\n\r\n")
            .map(|l| l.split("\r\n").collect())
            .collect();

        let mut printers: Vec<printer::Printer> = Vec::with_capacity(lines.len());

        for line in lines {
            let name = line[0].split(":").last().unwrap().trim();
            let driver_name = line[1].split(":").last().unwrap().trim();

            printers.push(printer::Printer::new(
                name.to_string(),
                name.to_string(),
                driver_name.to_string(),
                &self::print,
            ));
        }

        return printers;
    }

    return Vec::with_capacity(0);
}

/**
 * Print on windows using lpr
 */
pub fn print(printer_system_name: &str, file_path: &str) -> Result<bool, String> {
    let child = Command::new("powershell")
        .arg(format!(
            "Get-Content -Path \"{}\" |  Out-Printer -Name \"{}\"",
            file_path, printer_system_name
        ))
        .spawn()
        .unwrap();

    if child.id() > 0 {
        return Result::Ok(true);
    }

    return Result::Err("Failure to start print process".to_string());
}
