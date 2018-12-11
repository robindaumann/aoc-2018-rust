#[macro_use] extern crate text_io;
extern crate aoc;

use aoc::file_to_vec;
use std::cmp::max;
use std::collections::HashSet;

#[derive(Debug)]
struct Rect {
    id: u64,
    x: usize,
    y: usize,
    dx: usize,
    dy: usize,
}

fn main() {
    let path = "input/03.txt";
    let lines = file_to_vec(path).unwrap();

    let claims = claims(lines);
    let mut fabric = create_fabric(&claims);
    fill_fabric(&mut fabric, &claims);
    let mut overlaps: HashSet<u64> = HashSet::new();
    let square = count_overlaps(fabric, &mut overlaps);
    let all = create_set(&claims);

    let diff: Vec<&u64> = all.difference(&overlaps).collect();

    println!("Diff: {:?}", diff);
    
    println!("Overlaps: {}", square);
}

fn create_set(claims: &[Rect]) -> HashSet<u64> {
    let mut res = HashSet::new();
    for claim in claims {
        res.insert(claim.id);
    }
    res
}

fn count_overlaps (fabric: Vec<Vec<Vec<&Rect>>>, set: &mut HashSet<u64>) -> u64 {
    let mut count = 0;
    for vy in fabric {
        for claims in &vy {
            if claims.len() > 1 {
                for claim in claims {
                    set.insert(claim.id);
                }
                count += 1;
            }
        }
    }
    count
}

fn fill_fabric<'a>(fabric: &mut Vec<Vec<Vec<&'a Rect>>>, claims: &'a [Rect]) {
    for claim in claims {
        for xx in fabric.iter_mut().skip(claim.x).take(claim.dx) {
            for yy in xx.iter_mut().skip(claim.y).take(claim.dy) {
                yy.push(claim);
            }
        }
    }
}

fn create_fabric(claims: &[Rect]) -> Vec<Vec<Vec<&Rect>>> {
    let mut xmax = 0;
    let mut ymax = 0;


    for claim in claims {
        let xx = claim.x + claim.dx;
        let yy = claim.y + claim.dy;
        xmax = max(xx, xmax);
        ymax = max(yy, ymax);
    }

    let mut res = Vec::new();
    for _ in 0..xmax {
        let mut vx = Vec::new();
        for _ in 0..ymax {
            let vy = Vec::new();
            vx.push(vy);
        }
        res.push(vx);
    }
    res
}

fn claims(lines: Vec<String>) -> Vec<Rect> {
    let mut res = Vec::new();
    for line in lines {
        let (id,x,y,dx,dy) : (u64, usize, usize, usize, usize);
        scan!(line.bytes() => "#{} @ {},{}: {}x{}", id, x, y, dx, dy);
        res.push(Rect {id, x, y, dx, dy,})
    }
    res
}

