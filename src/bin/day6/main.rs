fn main() {
    let file = std::fs::read_to_string("./src/bin/day6/input.txt").unwrap();

    let races = parse_races(&file);
    let mut total_number = 0;
    for race in races {
        let winnings = race.get_possible_winnings();
        if total_number == 0 {
            total_number = winnings;
        } else {
            total_number *= winnings;
        }
    }
    println!("{total_number}");
    assert_eq!(6209190, total_number);

    let race = parse_races_part2(&file);
    let total_number = race.get_possible_winnings();
    println!("{total_number}");
    assert_eq!(28545089, total_number);
}

#[cfg(test)]
const EXAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200";
#[derive(Debug, PartialEq)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn get_possible_winnings(&self) -> usize {
        let mut winning_cnt = 0;
        for hold_time in 0..=self.time {
            let speed = hold_time; // hold time * 1ms = speed
            let distance = speed * (self.time - hold_time);
            if distance > self.distance {
                winning_cnt += 1;
            }
        }
        winning_cnt
    }
}

fn parse_races_part2(s: &str) -> Race {
    let mut lines = s.lines();
    let line_time = lines.next().unwrap();
    let line_dist = lines.next().unwrap();

    let mut times = line_time.split_inclusive("Time:");
    times.next().unwrap();
    let time = aoc_23::remove_non_numeric(times.next().unwrap())
        .parse::<usize>()
        .unwrap();

    let mut distances = line_dist.split_inclusive("Distance:");
    distances.next().unwrap();
    let distance = aoc_23::remove_non_numeric(distances.next().unwrap())
        .parse::<usize>()
        .unwrap();
    Race { time, distance }
}

fn parse_races(s: &str) -> Vec<Race> {
    let mut lines = s.lines();
    let line_time = lines.next().unwrap();
    let line_dist = lines.next().unwrap();

    let mut times = line_time.split_inclusive("Time:");
    times.next().unwrap();
    let mut time_numbers = Vec::new();
    for t in times {
        for i in t.split(' ') {
            if !i.is_empty() {
                let time = i.trim().parse::<usize>().unwrap();
                time_numbers.push(time);
            }
        }
    }

    let mut distances = line_dist.split_inclusive("Distance:");
    distances.next().unwrap();
    let mut distance_numbers = Vec::new();
    for d in distances {
        for i in d.split(' ') {
            if !i.is_empty() {
                let distance = i.trim().parse::<usize>().unwrap();
                distance_numbers.push(distance);
            }
        }
    }
    let mut races = Vec::new();
    if distance_numbers.len() == time_numbers.len() {
        for pair in distance_numbers.iter().zip(time_numbers.iter()) {
            races.push(Race {
                distance: *pair.0,
                time: *pair.1,
            });
        }
    }
    races
}

#[test]
fn test_parse_races() {
    let mut races = parse_races(EXAMPLE);
    assert_eq!(
        races.pop().unwrap(),
        Race {
            distance: 200,
            time: 30
        }
    );

    assert_eq!(
        races.pop().unwrap(),
        Race {
            distance: 40,
            time: 15
        }
    );

    assert_eq!(
        races.pop().unwrap(),
        Race {
            distance: 9,
            time: 7,
        }
    );
    let race = parse_races_part2(EXAMPLE);
    assert_eq!(
        race,
        Race {
            distance: 940200,
            time: 71530
        }
    );
}

#[test]
fn test_winning_cnt() {
    let race = Race {
        distance: 9,
        time: 7,
    };
    assert_eq!(race.get_possible_winnings(), 4);

    let mut races = parse_races(EXAMPLE);

    assert_eq!(races.pop().unwrap().get_possible_winnings(), 9);
    assert_eq!(races.pop().unwrap().get_possible_winnings(), 8);
    assert_eq!(races.pop().unwrap().get_possible_winnings(), 4);

    let race = parse_races_part2(EXAMPLE);
    assert_eq!(race.get_possible_winnings(), 71503);
}
