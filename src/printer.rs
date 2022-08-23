use std::env;
use std::fs::File;
use std::io::Write;

use uuid::Uuid;

#[derive(Debug)]
pub struct Job {
    pub status: JobStatus,
    pub error: Option<String>,
    pub file_path: String,
}

#[derive(Debug)]
pub enum JobStatus {
    SUCCESS,
    FAILED,
}

pub struct Printer {
    pub id: String,
    pub name: String,
    pub system_name: String,
    exec: &'static dyn Fn(&str, &str) -> Result<bool, String>,
}

impl std::fmt::Debug for Printer {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "Printer {{ id: {:?}, name: {:?}, system_name: {:?} }}", self.id, self.name, self.system_name)
    }
}

impl Printer {

    pub fn new(
        name: String,
        system_name: String,
        exec: &'static dyn Fn(&str, &str)-> Result<bool, String>,
    ) -> Printer {
        Printer {
            id: Uuid::new_v5(&Uuid::NAMESPACE_DNS, system_name.as_bytes()).to_string(),
            name,
            system_name,
            exec,
        }
    }


    /**
     * Print bytes into self printer instnace
     */
    pub fn print(&self, buffer: &[u8]) -> Job {

        let tmp_file_path = env::temp_dir().join(Uuid::new_v4().to_string());

        let mut tmp_file = File::create(&tmp_file_path).unwrap();
        let save = tmp_file.write(buffer);

        if save.is_err() {
            return Job {
                status: JobStatus::FAILED,
                error: Some(save.err().unwrap().to_string()),
                file_path: tmp_file_path.to_string_lossy().to_string()
            }
        }

        return _print(&self.system_name, tmp_file_path.as_os_str().to_str().unwrap(), &self.exec);

    }


    /**
     * Print specific file into self printer instnace
     */
    pub fn print_file(&self, file_path: &str) -> Job {
        return _print(&self.system_name, file_path, &self.exec);
    }
}


/**
 * General printer function - process any result into Job with status
 */
fn _print(printer_system_name: &str, file_path: &str, exec: &&'static dyn Fn(&str, &str)-> Result<bool, String>) -> Job {

    let print = exec(printer_system_name, file_path);

    if print.is_err() {
        return Job {
            status: JobStatus::FAILED,
            error: print.err(),
            file_path: file_path.to_string(),
        }
    }

    return Job {
        status: JobStatus::SUCCESS,
        error: None,
        file_path: file_path.to_string(),
    }

}
