#[cfg(target_family = "unix")]
use std::{env, fs::File, io::Write, path::PathBuf, time::{SystemTime, UNIX_EPOCH}};

#[cfg(target_family = "windows")]
use std::{fs::{metadata, File}, io::Read};

#[cfg(target_family = "windows")]
pub fn get_file_as_bytes(path: &str) -> Option<Vec<u8>> {
    let f = File::open(path);

    return if f.is_ok() {
        let metadata = metadata(path).unwrap();
        let mut buffer = vec![0; metadata.len() as usize];
        let result = f.unwrap().read(&mut buffer);
        if result.is_ok() {
            Some(buffer)
        } else {
            None
        }
    } else {
        None
    }
}

#[cfg(target_family = "unix")]
pub fn save_tmp_file(buffer: &[u8]) -> Option<PathBuf> {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();

    let file_path = env::temp_dir().join(time.to_string());

    let mut tmp_file = File::create(&file_path).unwrap();
    let save = tmp_file.write(buffer);

    return if save.is_ok() {
        Some(file_path)
    } else {
        None
    }
}