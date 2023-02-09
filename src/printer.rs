
use std::env;
use std::fs::File;
use std::io::Write;

use uuid::Uuid;


/**
 * Job is a result of an priting execution
 * When status is JobStatus::Failed, has too a error as String
 */
#[derive(Debug)]
pub struct Job {
    pub status: JobStatus,
    pub error: Option<String>,
    pub file_path: String,
}


/**
 * Enum of possible status of printing execution
 */
#[derive(Debug)]
pub enum JobStatus {
    SUCCESS,
    FAILED,
}

/**
 * Printer is a struct to representation the system printer
 * They has an ID composed by your system_name and has printing method to print directly
 */
pub struct Printer {

    /**
     * Uuid v5 of system_name with DNS namespace
     */
    pub id: String,

    /**
     * Visual reference of system printer name
     */
    pub name: String,

    /**
     * Name of Printer exactly as on system 
     */
    pub system_name: String,

    /**
     * Name of printer driver on system
     */
    pub driver_name: String,

    /**
     * A private reference of print command executor
     */
    exec: &'static dyn Fn(&str, &str) -> Result<bool, String>,

}

impl std::fmt::Debug for Printer {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "Printer {{ id: {:?}, name: {:?}, system_name: {:?}, driver_name: {:?} }}", self.id, self.name, self.system_name, self.driver_name)
    }
}

impl Clone for Printer {
    fn clone(&self) -> Printer {
        return Printer {
            id: self.id.clone(),
            name: self.name.clone(),
            exec: self.exec.clone(),
            system_name: self.system_name.clone(),
            driver_name: self.driver_name.clone(),
        }
    }
}


impl Printer {

    /**
     * Creates a new `Printer`
     */
    pub fn new(
        name: String,
        system_name: String,
        driver_name: String,
        exec: &'static dyn Fn(&str, &str)-> Result<bool, String>,
    ) -> Printer {
        Printer {
            id: Uuid::new_v5(&Uuid::NAMESPACE_DNS, system_name.as_bytes()).to_string(),
            name,
            system_name,
            driver_name,
            exec,
        }
    }


    /**
     * Print bytes with self printer instnace
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
     * Print specific file with self printer instnace
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
