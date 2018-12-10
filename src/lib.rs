use std::io::{BufRead, BufReader, Error};
use std::fs::File;

pub fn file_to_vec(path: &str) -> Result<Vec<String>, Error> {
    let f = File::open(path)?;
    let f = BufReader::new(f);
    f.lines().collect()
}
