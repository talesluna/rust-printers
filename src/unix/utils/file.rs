use std::{
    env,
    fs::File,
    io::Write,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

pub fn save_tmp_file(buffer: &[u8]) -> Option<PathBuf> {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();

    let file_path = env::temp_dir().join(time.to_string());

    let mut tmp_file = File::create(&file_path).unwrap();
    let save = tmp_file.write(buffer);

    return if save.is_ok() { Some(file_path) } else { None };
}
