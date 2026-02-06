use std::{
    fs::{File, metadata},
    io::Read,
};

#[cfg(target_family = "unix")]
use std::{
    env,
    io::Write,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::common::base::errors::PrintersError;

#[cfg(target_family = "unix")]
pub fn save_tmp_file(buffer: &[u8]) -> Result<PathBuf, PrintersError> {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(PrintersError::file_error)?
        .subsec_nanos();

    let file_path = env::temp_dir().join(time.to_string());

    File::create(&file_path)
        .map_err(PrintersError::file_error)?
        .write(buffer)
        .map(|_| file_path)
        .map_err(PrintersError::file_error)
}

pub fn get_file_as_bytes(path: &str) -> Result<Vec<u8>, PrintersError> {
    let metadata = metadata(path).map_err(PrintersError::file_error)?;
    let mut buffer = vec![0; metadata.len() as usize];

    File::open(path)
        .map_err(PrintersError::file_error)?
        .read(&mut buffer)
        .map_err(PrintersError::file_error)
        .map(|_| buffer)
}
