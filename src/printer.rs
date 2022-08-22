use std::env;
use std::fs::File;
use std::io::Write;

use uuid::Uuid;

#[derive(Debug)]
pub struct Job {
    pub status: JobStatus,
    pub error: Option<String>,
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
    executor: Box<dyn Fn(&str, &str) -> Result<bool, String>>,
}

impl Printer {

    pub fn new(
        name: String,
        system_name: String,
        executor: Box<dyn Fn(&str, &str)-> Result<bool, String>>,
    ) -> Printer {
        Printer {
            id: Uuid::new_v5(&Uuid::NAMESPACE_DNS, system_name.as_bytes()).to_string(),
            name,
            system_name,
            executor
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
            }
        }

        return _print(&self.system_name, tmp_file_path.as_os_str().to_str().unwrap(), &self.executor);

    }


    /**
     * Print specific file into self printer instnace
     */
    pub fn print_file(&self, file_path: &str) -> Job {
        return _print(&self.system_name, file_path, &self.executor);
    }
}


/**
 * General printer function - process any result into Job with status
 */
fn _print(printer_system_name: &str, file_path: &str, executor: &Box<dyn Fn(&str, &str)-> Result<bool, String>>) -> Job {

    let print = executor(printer_system_name, file_path);

    if print.is_err() {
        return Job {
            status: JobStatus::FAILED,
            error: print.err(),
        }
    }

    return Job {
        status: JobStatus::SUCCESS,
        error: None,
    }

}
