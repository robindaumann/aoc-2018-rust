#[macro_use]
extern crate text_io;
extern crate aoc;
extern crate chrono;

use chrono::format::ParseError;
use chrono::prelude::*;
use std::collections::HashMap;

fn main() {
    let path = "input/04.txt";

    let mut lines = aoc::file_to_vec(path).unwrap();
    lines.sort();

    let events = parse_events(&lines).unwrap();

    let guards = build_guards(events).unwrap();

    let (id, minute) = find_strategy1(&guards);
    println!("Strategy1 product: {}", id as usize * minute);

    let (id, minute) = find_strategy2(&guards);
    println!("Strategy2 product: {}", id as usize * minute);
}

fn find_strategy2(guards: &HashMap<u64, Vec<u32>>) -> (u64, usize) {
    let mut id = 0;
    let mut minute = 0;

    let mut max = 0;
    for (key, val) in guards {
        let (c_minute,count) = find_max(val.as_slice());

        if count > max {
            max = count;
            id = *key;
            minute = c_minute;
        }
    }

    (id, minute)
}

fn find_strategy1(guards: &HashMap<u64, Vec<u32>>) -> (u64, usize) {
    let mut id = 0;
    let mut minute = 0;

    let mut max = 0;
    for (c_id, minutes) in guards {
        let sum: u32 = minutes.iter().sum();
        if sum > max {
            id = *c_id;
            minute = find_max(minutes).0;
            max = sum;
        }
    }

    (id, minute)
}

fn find_max(arr: &[u32]) -> (usize,u32) {
    let mut count_max = 0;
    let mut res_idx = 0;
    for (idx, count) in arr.iter().enumerate() {
        if *count > count_max {
            res_idx = idx;
            count_max = *count;
        }
    }
    (res_idx,count_max)
}

fn build_guards(events: Vec<Event>) -> Option<HashMap<u64, Vec<u32>>> {
    let mut res = HashMap::new();
    let mut cur_id = 0u64;
    let mut cur_min = 0u32;
    for event in events {
        match event.kind {
            EventType::Start => {
                cur_id = read!("Guard #{} begins shift", event.text.bytes());
                if res.get(&cur_id) == None {
                    let vec = [0; 60].to_vec();
                    res.insert(cur_id, vec);
                }
            }
            EventType::Awake => {
                for idx in cur_min..event.dt.minute() {
                    let vec = res.get_mut(&cur_id)?;
                    vec[idx as usize] += 1;                   
                }
            }
            EventType::Sleep => {
                cur_min = event.dt.minute();
            }
        }
    }
    Some(res)
}

fn parse_events(lines: &[String]) -> Result<Vec<Event>, ParseError> {
    let mut res = Vec::new();
    for line in lines {
        res.push(Event::from_string(line.as_str())?);
    }
    Ok(res)
}

enum EventType {
    Start,
    Sleep,
    Awake,
}

struct Event {
    kind: EventType,
    dt: NaiveDateTime,
    text: String,
}

impl Event {
    fn from_string(s: &str) -> Result<Event, ParseError> {
        let (dt, text): (String, String);
        scan!(s.bytes() => "[{}] {}\n", dt, text);

        let dt = NaiveDateTime::parse_from_str(dt.as_str(), "%Y-%m-%d %H:%M")?;
        let kind = match text.as_str() {
            "falls asleep" => EventType::Sleep,
            "wakes up" => EventType::Awake,
            _ => EventType::Start,
        };

        Ok(Event { kind, dt, text })
    }
}
