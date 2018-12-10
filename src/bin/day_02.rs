extern crate aoc;

use aoc::file_to_vec;
use std::collections::HashMap;
use std::io::Error;

fn main() {
    let lines = file_to_vec("input/02.txt").unwrap();
    // Task 1
    let res = calc_checksum(&lines).unwrap();
    println!("Checksum: {}", res);

    // Task 2
    let res = find_same(&lines);
    match res {
        None => println!("No common pair"),
        Some(x) => {
            println!("Common letters: {}", x)
        },
        
    }
}

fn find_same(lines: &Vec<String>) -> Option<String> {
    for line in lines {
        for sec in lines {
            if sec == line {
                continue;
            }
            let com = comp(&line, &sec);
            if com.len() == sec.len()-1 {
                return Some(com)
            }
        }
    }
    None
}

fn comp(line: &String, other: &String) -> String {
    let mut res = String::new();
    for(c1, c2) in line.chars().zip(other.chars()) {
        if c1 == c2 {
            res.push(c1);
        }
    }
    res
}

fn calc_checksum(lines: &Vec<String>) -> Result<u64, Error> {
    let mut threes = 0;
    let mut twos = 0;
    for line in lines {
        let mut cs = HashMap::new();
        for c in line.chars() {
            let count = cs.entry(c).or_insert(0);
            *count += 1;
        }
        let mut inc_two = 0;
        let mut inc_three = 0;
        for count in cs.values() {
            match count {
                2 => inc_two = 1,
                3 => inc_three = 1,
                _ => (),
            } 
        }
        twos += inc_two;
        threes += inc_three;
    }
    Ok(twos*threes)
}
