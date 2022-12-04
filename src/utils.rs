use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn char_int(c: char, zero_char : char) -> u32 {
    c as u32 - zero_char as u32
}