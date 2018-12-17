extern crate aoc;

fn main() {
    let path = "input/05.txt";
    let input = aoc::file_to_string(path).unwrap();

    let res;
    match react(input.as_str()) {
        Some(x) => res = x,
        None => res = 0,
    }
    println!("Remaining size: {}", res);

    let res;
    match optimize(input.as_str()) {
        Some(x) => res = x,
        None => res = 0,
    }
    println!("Optimized size: {}", res);
}

fn optimize(input: &str) -> Option<usize>{
    let mut min = std::usize::MAX;

    for b in b'a'..=b'z' {
        let ch = b as char;
        let mut strip = ch.to_string();
        strip.push(ch.to_ascii_uppercase());

        let s = strip_chars(input, strip.as_str());

        let size = react(s.as_str())?;
        if size < min {
            min = size;
        }
    }

    Some(min)
}

fn strip_chars(original : &str, to_strip : &str) -> String {
    original.chars().filter(|&c| !to_strip.contains(c)).collect()
}

fn react(input: &str) -> Option<usize> {
    let mut res = input.to_owned();
    let mut size = res.len();
    loop {
        res = step(res)?;
        let next_size = res.len();
        if next_size == size {
            break;
        }
        size = next_size;
    }
    Some(res.len())
}

fn step(input: String) -> Option<String> {
    let mut res = String::new();
    let mut iter = input.chars();
    let mut prev = iter.next()?;

    let mut skip = false;
    for c in iter {
        if skip {
            skip = false;
        } else if equiv(c, prev) {
            skip = true;
        } else {
            res.push(prev);
        }
        prev = c;
    }

    if !skip {
        res.push(prev);
    }

    Some(res)
}

fn equiv(c1: char, c2: char) -> bool {
    let eq = c1.to_ascii_lowercase() == c2.to_ascii_lowercase();
    let uplow = c1.is_ascii_uppercase() && c2.is_ascii_lowercase();
    let lowup = c1.is_ascii_lowercase() && c2.is_ascii_uppercase();
    eq && (uplow || lowup)
}


