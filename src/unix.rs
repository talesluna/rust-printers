use uuid::Uuid;
use std::process::Command;    

/**
 * Get printers on unix systems using lpstat
 */
pub fn get_printers() -> Vec<super::Printer> {
    let out = Command::new("lpstat").arg("-e").output().unwrap();
    if out.status.success() {
        unsafe {

            let out_str = String::from_utf8_unchecked(out.stdout);
            let lines: Vec<&str> = out_str.split_inclusive("\n").collect();
            let mut printers: Vec<super::Printer> = Vec::with_capacity(lines.len());

            for line in lines {
                let system_name = line.replace("\n", "");
                printers.push(super::Printer {
                    id: Uuid::new_v5(&Uuid::NAMESPACE_DNS, system_name.as_bytes()).to_string(),
                    name: String::from(system_name.replace("_", " ").trim()),
                    system_name: system_name,
                });
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
pub fn print(printer_system_name: &String, file_path: &std::path::PathBuf) -> bool {
    let process = Command::new("lp")
    .arg("-d")
    .arg(printer_system_name)
    .arg(file_path)
    .output()
    .unwrap();

    return process.status.success();

}

