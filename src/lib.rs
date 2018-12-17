use std::io::{BufRead, BufReader, Read, Error};
use std::fs::File;

pub fn file_to_vec(path: &str) -> Result<Vec<String>, Error> {
    let f = File::open(path)?;
    let f = BufReader::new(f);
    f.lines().collect()
}

pub fn file_to_string(path: &str) -> Result<String, Error> {
    let mut f = File::open(path)?;
    let mut res = String::new();
    f.read_to_string(&mut res)?;
    Ok(res)
}
