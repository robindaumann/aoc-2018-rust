extern crate aoc;
#[macro_use]
extern crate text_io;

use std::collections::HashMap;

fn main() {
    let path = "input/06.test.txt";

    let input = aoc::file_to_vec(path).unwrap();

    let points = Points::parse(input);

    println!("dx:{}, dy:{}, next:{}", points.dx, points.dy, points.next);

    let mut grid = Grid::init(points.dx, points.dy);
    grid.fill(&points);
    grid.calc_distances(&points).unwrap();

    for vy in grid.grid {
        for p in vy {
            print!("{},", p);
        }
        println!();
    }
}

struct Point {
    x: usize,
    y: usize,
    id: u64,
}

impl Point {
    fn dist(&self, x: usize, y: usize) -> i64 {
        ((self.x as i64 - x as i64).abs() + (self.y as i64 - y as i64).abs())
    }
}

struct Points {
    points: HashMap<u64, Point>,
    next: u64,
    dx: usize,
    dy: usize,
}

impl Points {
    fn parse(input: Vec<String>) -> Points {
        let mut res = Points {
            points: HashMap::new(),
            next: 1,
            dx: 0,
            dy: 0,
        };

        for line in input {
            let (x, y): (usize, usize);
            scan!(line.bytes() => "{}, {}", x, y);
            res.push(x, y);
        }
        res
    }

    fn push(&mut self, x: usize, y: usize) {
        if x > self.dx {
            self.dx = x;
        }
        if y > self.dy {
            self.dy = y;
        }

        let id = self.next;
        self.next += 1;

        self.points.insert(id, Point { x, y, id });
    }

    fn min_point(&self, x: usize, y: usize) -> Option<u64> {
        let mut min = std::i64::MAX;
        let mut id = None;
        for point in self.points.values() {
            let dist = point.dist(x, y);
            // Multiple points share the smallest distance
            if dist == min && id != None && id? != point.id {
                return Some(0);
            }

            if dist < min {
                min = dist;
                id = Some(point.id);
            }
        }
        id
    }
}

struct Grid {
    grid: Vec<Vec<u64>>,
}

impl Grid {
    fn init(dx: usize, dy: usize) -> Grid {
        let mut grid = Vec::new();
        for _ in 0..=dy + 1 {
            let mut v = Vec::new();
            for _ in 0..=dx + 1 {
                v.push(0);
            }
            grid.push(v)
        }
        Grid { grid }
    }

    fn fill(&mut self, points: &Points) {
        for point in points.points.values() {
            self.grid[point.y][point.x] = point.id;
        }
    }

    fn calc_distances(&mut self, points: &Points) -> Option<()> {
        for (y, v) in self.grid.iter_mut().enumerate() {
            for (x, id) in v.iter_mut().enumerate() {
                *id = points.min_point(x, y)?;
            }
        }
        Some(())
    }
}
