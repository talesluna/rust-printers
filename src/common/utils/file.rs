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

#[cfg(target_family = "unix")]
pub fn save_tmp_file(buffer: &[u8]) -> Result<PathBuf, String> {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();

    let file_path = env::temp_dir().join(time.to_string());

    File::create(&file_path)
        .map_err(|err| format!("Create temp file failed: {err}"))?
        .write(buffer)
        .map(|_| file_path)
        .map_err(|err| format!("Write temp file failed: {err}"))
}

pub fn get_file_as_bytes(path: &str) -> Result<Vec<u8>, String> {
    let metadata = metadata(path).map_err(|err| format!("Read file metadata failed: {err}"))?;
    let mut buffer = vec![0; metadata.len() as usize];

    File::open(path)
        .map_err(|err| format!("Open file failed: {err}"))?
        .read(&mut buffer)
        .map_err(|err| format!("Read file file failed: {err}"))
        .map(|_| buffer)
}
