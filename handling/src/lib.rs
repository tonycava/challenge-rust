use std::fs;
use std::fs::File;
use std::io::{Write};

pub fn open_or_create(file: &str, content: &str) {
    let str = fs::read_to_string(file).unwrap_or("".to_string());
    if str != "" {
        println!("{str}");
        return;
    }
    File::create(file)
        .expect("Error while creating the file")
        .write_all(content.as_bytes())
        .expect("Error while writing to file");
}