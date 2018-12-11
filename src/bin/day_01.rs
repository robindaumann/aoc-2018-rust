extern crate aoc;

use aoc::file_to_vec;
use std::collections::HashSet;
use std::io::Error;

fn main() {
    let path = "input/01.txt";
    let lines = file_to_vec(path).unwrap();
    let res = read(&lines).unwrap();

    println!("Sum: {}", res.0);
    match res.1 {
        Some(seen) => println!("First duplicate seen frequency: {}", seen),
        None => println!("No duplicate found."),
    }
}

fn read(lines: &[String]) -> Result<(i64, Option<i64>), Error> {
    let mut set = HashSet::new();
    let mut sum = 0;
    set.insert(sum);
    let mut seen = None;
    for line in lines {
        if set.contains(&sum) {
            seen = Some(sum);
        }
        let num: i64 = line.parse().expect("Could not parse!");
        sum += num;
        set.insert(sum);
    }
    Ok((sum, seen))
}
