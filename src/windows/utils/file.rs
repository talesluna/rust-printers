use std::{
    fs::{metadata, File},
    io::Read,
};

pub fn get_file_as_bytes(path: &str) -> Option<Vec<u8>> {
    let f = File::open(path);

    if f.is_ok() {
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
