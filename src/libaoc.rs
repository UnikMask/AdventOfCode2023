use std::{fs, vec::Vec};

pub fn load_file(fp: &str) -> Vec<String> {
    fs::read_to_string(fp)
        .expect("File doesn't exist!")
        .split("\n")
        .map(|s| s.to_owned())
        .collect()
}
