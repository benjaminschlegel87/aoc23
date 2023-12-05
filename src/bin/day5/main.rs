use std::{borrow::BorrowMut, ops::Range, sync::Mutex};
fn main() {
    let file = std::fs::read_to_string("./src/bin/day5/input.txt").unwrap();
    // parse input file to type holding all relevant informations integers

    let line = file.lines().next().unwrap();
    let seeds = parse_seed_numbers(line).unwrap();
    let all_maps = split_in_maps(&file);
    let mut min_location: Vec<usize> = Vec::new();
    let mut handles = Vec::new();

    for s in seeds {
        let maps = all_maps.clone();
        handles.push(std::thread::spawn(move || calc_seed(s, maps)));
    }
    for handle in handles.into_iter() {
        min_location.push(handle.join().unwrap());
    }
    let res = min_location.into_iter().min().unwrap();
    assert!(res == 323142486);
}

fn calc_seed(seed: usize, mut all_maps: Vec<Vec<Map>>) -> usize {
    println!("Working on {seed}");
    let location_map = all_maps.pop().unwrap();
    let humidity_map = all_maps.pop().unwrap();
    let temperature_map = all_maps.pop().unwrap();
    let light_map = all_maps.pop().unwrap();
    let water_map = all_maps.pop().unwrap();
    let fertalizer_map = all_maps.pop().unwrap();
    let soil_map = all_maps.pop().unwrap();
    let soil = transform_by_map(seed, soil_map.as_ref());
    let fertalizer = transform_by_map(soil, fertalizer_map.as_ref());
    let water = transform_by_map(fertalizer, water_map.as_ref());
    let light = transform_by_map(water, light_map.as_ref());
    let temp = transform_by_map(light, temperature_map.as_ref());
    let humidity = transform_by_map(temp, humidity_map.as_ref());
    let location = transform_by_map(humidity, location_map.as_ref());
    eprintln!("LOCATION: {location}");
    location
}

fn parse_seed_numbers(s: &str) -> Option<Vec<usize>> {
    if let Some(index) = s.find("seeds:") {
        let (_rest, numbers) = s.split_at(index + "seeds:".len());
        let mut seeds = Vec::new();
        for number in numbers.split(' ') {
            if !number.is_empty() {
                seeds.push(number.trim().parse::<usize>().unwrap());
            }
        }
        Some(seeds)
    } else {
        None
    }
}
#[derive(Debug, Clone)]
struct Map {
    dest: usize,
    source: usize,
    range: usize,
}

fn split_in_maps(s: &str) -> Vec<Vec<Map>> {
    let mut all_maps = Vec::new();
    for (i, map_string) in s.split("map:").enumerate() {
        if i != 0 {
            let mut str = map_string.to_string();
            str.retain(|c| if c.is_ascii_alphabetic() { false } else { true });

            let str = str.trim();
            let mut maps = Vec::new();
            for n in str.lines() {
                if !n.is_empty() {
                    let c = n.chars().next().unwrap();
                    if c.is_numeric() {
                        // erstes ziechen is eine Zahl
                        let mut map = Map {
                            dest: 0,
                            source: 0,
                            range: 0,
                        };
                        let mut numb = n.split(' ');
                        map.dest = numb.next().unwrap().parse::<usize>().unwrap();
                        map.source = numb.next().unwrap().parse::<usize>().unwrap();
                        map.range = numb.next().unwrap().parse::<usize>().unwrap();
                        maps.push(map);
                    }
                }
            }
            all_maps.push(maps);
        } else {
        }
    }
    all_maps
}
#[test]
fn test_split_in_maps() {
    let all_maps = split_in_maps(EXAMPLE);
    eprintln!("{:?}", all_maps);
    assert_eq!(all_maps.len(), 7);
    assert_eq!(all_maps.get(0).unwrap().len(), 2);
    assert_eq!(all_maps.get(1).unwrap().len(), 3);
    assert_eq!(all_maps.get(2).unwrap().len(), 4);
    assert_eq!(all_maps.get(3).unwrap().len(), 2);
    assert_eq!(all_maps.get(4).unwrap().len(), 3);
    assert_eq!(all_maps.get(5).unwrap().len(), 2);
    assert_eq!(all_maps.get(6).unwrap().len(), 2);
}
#[test]
fn test_parse_seeds() {
    let line = EXAMPLE.lines().next().unwrap();
    let mut res = parse_seed_numbers(line).unwrap();
    assert_eq!(13, res.pop().unwrap());
    assert_eq!(55, res.pop().unwrap());
    assert_eq!(14, res.pop().unwrap());
    assert_eq!(79, res.pop().unwrap());
}

#[test]
fn test_parse_all() {
    let line = EXAMPLE.lines().next().unwrap();
    let mut seeds = parse_seed_numbers(line).unwrap();
    let mut all_maps = split_in_maps(EXAMPLE);

    assert_eq!(13, seeds.pop().unwrap());
    assert_eq!(55, seeds.pop().unwrap());
    assert_eq!(14, seeds.pop().unwrap());
    assert_eq!(79, seeds.pop().unwrap());

    assert_eq!(all_maps.len(), 7);
    assert_eq!(all_maps.get(0).unwrap().len(), 2);
    assert_eq!(all_maps.get(1).unwrap().len(), 3);
    assert_eq!(all_maps.get(2).unwrap().len(), 4);
    assert_eq!(all_maps.get(3).unwrap().len(), 2);
    assert_eq!(all_maps.get(4).unwrap().len(), 3);
    assert_eq!(all_maps.get(5).unwrap().len(), 2);
    assert_eq!(all_maps.get(6).unwrap().len(), 2);
    let location_map = all_maps.pop();
    let humidity_map = all_maps.pop();
    let temperature_map = all_maps.pop();
    let light_map = all_maps.pop();
    let water_map = all_maps.pop();
    let fertalizer_map = all_maps.pop();
    let soil_map = all_maps.pop();
    // Für jeden Seed gehe ich die Maps durch und ermittle die Location
    // die niedrigste Location ist das Ziel
    let line = EXAMPLE.lines().next().unwrap();
    let seeds = parse_seed_numbers(line).unwrap();
    let mut min_location = Vec::new();
    for s in seeds {
        let soil = transform_by_map(s, soil_map.as_ref().unwrap());
        eprintln!("{soil}");
        let fertalizer = transform_by_map(soil, fertalizer_map.as_ref().unwrap());
        eprintln!("{fertalizer}");
        let water = transform_by_map(fertalizer, water_map.as_ref().unwrap());
        eprintln!("{water}");
        let light = transform_by_map(water, light_map.as_ref().unwrap());
        eprintln!("{light}");
        let temp = transform_by_map(light, temperature_map.as_ref().unwrap());
        eprintln!("{temp}");
        let humidity = transform_by_map(temp, humidity_map.as_ref().unwrap());
        eprintln!("{humidity}");
        let location = transform_by_map(humidity, location_map.as_ref().unwrap());
        min_location.push(location);
    }
    assert_eq!(min_location.into_iter().min().unwrap(), 35);
}

fn transform_by_map(val: usize, map: &Vec<Map>) -> usize {
    let mut dest = None;
    for m in map {
        // This contains is very important for getting this computed in a reasonable time
        // iterating over all ranges takes forever
        if (m.source..(m.source + m.range)).contains(&val) {
            let s = (m.source..(m.source + m.range)).find(|x| *x == val);
            if let Some(num) = s {
                if dest.is_none() {
                    // val is in the source range of this map
                    // look up
                    dest = Some(m.dest + (num - m.source));
                    break;
                } else {
                    panic!("Should not happen that we find two matches");
                }
            }
        } else {
        }
    }
    if dest.is_none() {
        // No match => 1:1
        dest = Some(val);
    }

    dest.unwrap()
}

fn search_seed_dest(mut range: Range<usize>, val: usize) -> Option<usize> {
    let s = range.find(|x| *x == val);
    if let Some(num) = s {
        Some(num)
    } else {
        None
    }
}

#[cfg(test)]
const EXAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
