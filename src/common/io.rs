use std::{
    vec::Vec,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    
    return buf.lines().map(|l| l.expect("Could not parse line")).collect();
}