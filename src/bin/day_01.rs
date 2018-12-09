use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() {
    let res = read().unwrap();

    println!("Sum: {}", res.0);
    match res.1 {
        Some(seen) => println!("First duplicate seen frequency: {}", seen),
        None => println!("No duplicate found."),
    }
}

fn read() -> Result<(i64, Option<i64>), Error> {
    let f = File::open("input/01.txt")?;
    let f = BufReader::new(f);

    let mut set = HashSet::new();
    let mut sum = 0;
    set.insert(sum);
    let mut seen = None;
    for line in f.lines() {
        if set.contains(&sum) {
            seen = Some(sum);
        }
        let num: i64 = line?.parse().expect("Could not parse!");
        sum += num;
        set.insert(sum);
    }
    Ok((sum, seen))
}
