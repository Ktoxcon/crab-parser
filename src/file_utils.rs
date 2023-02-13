use std::{
    fs::File,
    io::{BufReader, Read},
};

pub fn get_file_content(file_path: String) -> Option<String> {
    let file = File::open(file_path).expect("Error trying to open file in path: {path}");
    let mut file_reader = BufReader::new(file);

    let mut file_content = String::new();

    file_reader
        .read_to_string(&mut file_content)
        .expect("Error reading file content.");

    Some(file_content)
}
