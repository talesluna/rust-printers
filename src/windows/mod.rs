#[cfg_attr(analyzer, allow(dead_code))]

use crate::{printer, shared};
use std::process::Command;

pub mod winspool;

/**
 * Get printers on windows using wmic
 */
pub fn get_printers() -> Vec<printer::Printer> {
    return winspool::enum_printers()
        .iter()
        .map(|info| {
            let name = shared::strings::string_from_wchar_t(info.pPrinterName);
            let driver_name = shared::strings::string_from_wchar_t(info.pDriverName);
            return printer::Printer::new(name.clone(), name.clone(), driver_name, &self::print);
        })
        .collect();
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
