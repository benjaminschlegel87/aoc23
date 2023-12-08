use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

fn main() {
    let file = std::fs::read_to_string("./src/bin/day8/input.txt").unwrap();
    //let steps = solve_part1(&file);
    //println!("{steps}");
    //assert_eq!(steps, 21409);

    let steps = solve_part_2_mt(&file);
    println!("{steps}");
    assert_eq!(steps, 21409);
}
#[derive(Debug, Clone)]
struct Map {
    positions: Vec<Position>,
    instructions: Vec<bool>,
}
fn get_all_positions(s: &str) -> Map {
    let mut lines = s.lines();
    let instructions = parse_instructions(lines.next().unwrap());
    lines.next().unwrap(); // ignore line
    let mut positions = Vec::new();
    for l in lines {
        positions.push(Position::new(l));
    }
    Map {
        positions,
        instructions,
    }
}

fn solve_part_2_smart(s: &str) -> usize {
    // Idea: At some point every path will repeat itself
    // we got 6 paths from 6 starting points of type __A
    // map the index of every path were we find __Z until we are back at our start value
    // take the index of all 6 paths
    // find matching step number that brings all 6 paths on a __Z index
    let mut steps = 0;

    steps
}
// Brute Force does not work - Still learned something about scoped threads ... :)
fn solve_part_2_mt(s: &str) -> usize {
    let map = get_all_positions(s);
    let instructions = map.instructions.clone();
    let positions = map.positions.clone();
    let starts = find_starting_points(map.positions);
    println!("Starts {:?}", starts);
    let mut start_pos = Vec::new();
    for s in starts {
        start_pos.push(s.pos);
    }

    let mut steps = 0;
    'outer: loop {
        for is_left in &instructions {
            let (done, ne) = std::thread::scope(|s| {
                let mut handles = Vec::new();
                for ne in start_pos.iter() {
                    let pos = positions.clone();
                    handles.push(s.spawn(move || {
                        let a = pos.iter().find(|pos| &pos.pos == ne).unwrap();
                        if *is_left {
                            a.next_pos.0.clone()
                        } else {
                            a.next_pos.1.clone()
                        }
                    }));
                }
                let mut next_pos = Vec::new();
                for h in handles {
                    next_pos.push(h.join().unwrap());
                }
                let mut is_done = true;
                for n in &next_pos {
                    if !n.ends_with('Z') {
                        is_done = false;
                    }
                }
                (is_done, next_pos)
            });
            steps += 1;
            if done {
                break 'outer;
            }
            start_pos = ne;
        }
    }
    steps
}
fn solve_part_2(s: &str) -> usize {
    let map = get_all_positions(s);
    let instructions = map.instructions.clone();
    let mut positions = map.positions.clone();
    let starts = find_starting_points(map.positions);
    println!("{:?}", starts);
    // angepasste Logik
    let mut next = starts.clone();
    let mut steps = 0;

    'outer: loop {
        // Loop over the instructions until a match is found
        for is_left in &instructions {
            let mut z_found = true;
            for n in &mut next {
                let a = positions.iter().find(|pos| pos.pos == n.pos).unwrap();

                if !a.pos.ends_with('Z') {
                    z_found = false;
                }
                if *is_left {
                    n.pos = a.next_pos.0.clone();
                } else {
                    n.pos = a.next_pos.1.clone();
                }
            }
            if z_found {
                break 'outer;
            } else {
                steps += 1;
            }
        }
    }
    steps
}

fn solve_part1(s: &str) -> usize {
    let map = get_all_positions(s);
    let mut next = "AAA";
    let mut steps = 0;
    'outer: loop {
        for is_left in &map.instructions {
            let a = map.positions.iter().find(|pos| pos.pos == next).unwrap();
            if a.pos != "ZZZ" {
                steps += 1;
                if *is_left {
                    next = &a.next_pos.0;
                } else {
                    next = &a.next_pos.1;
                }
            } else {
                break 'outer;
            }
        }
    }
    steps
}

#[cfg(test)]
const EXAMPLE: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
// bool is_left
fn parse_instructions(s: &str) -> Vec<bool> {
    let mut res = Vec::new();
    for c in s.chars() {
        if c == 'L' {
            res.push(true);
        } else {
            res.push(false);
        }
    }
    res
}
#[derive(Debug, Clone)]
struct Position {
    pos: String,
    next_pos: (String, String),
}

impl Position {
    fn new(s: &str) -> Self {
        let mut pos = Position {
            pos: String::new(),
            next_pos: (String::new(), String::new()),
        };
        let mut chars = s.chars();
        pos.pos.push(chars.next().unwrap());
        pos.pos.push(chars.next().unwrap());
        pos.pos.push(chars.next().unwrap());
        chars.next().unwrap();
        chars.next().unwrap();
        chars.next().unwrap();
        chars.next().unwrap();
        pos.next_pos.0.push(chars.next().unwrap());
        pos.next_pos.0.push(chars.next().unwrap());
        pos.next_pos.0.push(chars.next().unwrap());
        chars.next().unwrap();
        chars.next().unwrap();
        pos.next_pos.1.push(chars.next().unwrap());
        pos.next_pos.1.push(chars.next().unwrap());
        pos.next_pos.1.push(chars.next().unwrap());
        pos
    }
    fn get_next_pos(&self, is_left: bool) -> String {
        if is_left {
            self.next_pos.0.clone()
        } else {
            self.next_pos.1.clone()
        }
    }
}

fn find_starting_points(pos: Vec<Position>) -> Vec<Position> {
    let mut start_pos = Vec::new();
    for p in pos {
        if p.pos.ends_with('A') {
            start_pos.push(p);
        }
    }
    start_pos
}
#[cfg(test)]
const INSTRUCTIONS: &str = "LRLRRRLRRRLLLRLRRLLRLRRRLRLRRRLRLRRRLRLRRRLRRRLRLLRRRLRLRLRRLRRLRLRRLRRLRRLLRRRLRRRLRRLRRLRRLRRRLLRRLRLRRLRLRRLRRLRLRRLRRLLRLRRRLRRLRRRLLRLRLRLLRLLRLLRLRRLLRRLRLRLRRLRLLRRRLLRRRLRRLLRRRLRRRLRLRRRLLRRRLRLRRRLLLRRRLRLRLRRRLRRRLRRRLRLRRLLLRRLRRRLLRLRRRLRLRLLLRRLRLRRRLRLRRRR";

#[test]
fn test_new_position() {
    let pos = Position::new("GNK = (LBV, QNP)");
    assert_eq!(pos.pos, "GNK");
    assert_eq!(pos.next_pos.0, "LBV");
    assert_eq!(pos.next_pos.1, "QNP");
}

#[test]
fn test_get_next_pos() {
    let pos = Position::new("GNK = (LBV, QNP)");
    assert_eq!(pos.pos, "GNK");
    assert_eq!(pos.next_pos.0, "LBV");
    assert_eq!(pos.next_pos.1, "QNP");

    assert_eq!("LBV", pos.get_next_pos(true));
    assert_eq!("QNP", pos.get_next_pos(false));
}

#[test]
fn test_parse_example() {
    assert_eq!(2, solve_part1(EXAMPLE));
}

#[cfg(test)]
const EXAMPLE_PART2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

#[test]
fn test_find_starting_pos() {
    let steps = solve_part_2_mt(EXAMPLE_PART2);
    assert_eq!(6, steps);
}
