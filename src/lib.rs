use std::env;
use uuid::Uuid;
use std::fs::File;
use std::io::Write;

mod unix;
mod windows;

#[derive(Debug)]
pub struct Printer {
    pub id: String,
    name: String,
    pub system_name: String,
}

#[derive(Debug)]
pub struct Job {
    id: String,
    status: JobStatus,
    printer_id: String,
}

#[derive(Debug)]
pub enum JobStatus {
    SUCCESS,
    FAILED,
}

impl Job {

    /**
     * Print job
     */
    pub fn print(printer: &Printer, buffer: &[u8]) -> Job {

        let id = Uuid::new_v4().to_string();
        let tmp_file_path = env::temp_dir().join(&id);

        let mut tmp_file = File::create(&tmp_file_path).unwrap();
        tmp_file.write(buffer).unwrap();

        let has_queued: bool;

        if cfg!(windows) {
            has_queued = windows::print(&printer.system_name, &tmp_file_path);
        } else if cfg!(unix) {
            has_queued = unix::print(&printer.system_name, &tmp_file_path);
        } else {
            panic!("Unsupported OS");
        };

        return Job {
            id,
            status: if has_queued { JobStatus::SUCCESS } else { JobStatus::FAILED },
            printer_id: String::from(&printer.id),
        }
    }

}

impl Printer {

    /**
     * Return all printers on system
     */
    pub fn get_printers() -> Vec<Printer> {

        if cfg!(windows) {
            return windows::get_printers();
        }

        if cfg!(unix) {
            return unix::get_printers();
        }

        panic!("Unsupported OS");

    }

}
