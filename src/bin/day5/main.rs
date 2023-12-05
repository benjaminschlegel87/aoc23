fn main() {
    let file = std::fs::read_to_string("./src/bin/day5/input.txt").unwrap();
    // parse input file to type holding all relevant informations integers

    let line = file.lines().next().unwrap();
    let seeds = parse_seed_numbers(line).unwrap();
    let all_maps = split_in_maps(&file);
    let mut min_location: Vec<usize> = Vec::new();
    let mut handles = Vec::new();
    let mut seed_range = Vec::new();
    let mut start = None;
    // PART 1
    for s in seeds {
        let maps = all_maps.clone();
        handles.push(std::thread::spawn(move || calc_seed(s, &maps)));
    }
    for handle in handles.into_iter() {
        min_location.push(handle.join().unwrap());
    }
    let res = min_location.into_iter().min().unwrap();
    assert!(res == 323142486);

    // PART 2
    let seeds = parse_seed_numbers(line).unwrap();
    let mut handles = Vec::new();
    let mut min_location: Vec<usize> = Vec::new();
    for s in &seeds {
        if start.is_none() {
            start = Some(*s);
        } else {
            seed_range.push(start.unwrap()..(start.unwrap() + *s));
            start = None;
        }
    }
    println!(" Seed range {:?}", seed_range);

    for (num, s) in seed_range.into_iter().enumerate() {
        let maps = all_maps.clone();
        let len = s.len();
        handles.push(std::thread::spawn(move || {
            println!("Spawned Range {num} with total len {len}");
            //let mut min = Vec::new();
            let mut res = usize::MAX;
            for (n, i) in s.clone().enumerate() {
                //   min.push(calc_seed(i, m));
                let temp = calc_seed(i, &maps);
                if temp < res {
                    res = temp;
                }
                if n % 10_000_000 == 0 {
                    println!("Done {n}/{len} seeds in Range {num}");
                }
            }
            //let min = min.into_iter().min().unwrap();
            println!("Finished Range {num} with {}", res);
            //min
            res
        }));
    }
    for handle in handles.into_iter() {
        min_location.push(handle.join().unwrap());
    }
    let res = min_location.into_iter().min().unwrap();
    println!("Result {res}");
    assert!(res == 79874951)
}

fn calc_seed(seed: usize, all_maps: &[Vec<Map>]) -> usize {
    let location_map = all_maps.get(6).unwrap();
    let humidity_map = all_maps.get(5).unwrap();
    let temperature_map = all_maps.get(4).unwrap();
    let light_map = all_maps.get(3).unwrap();
    let water_map = all_maps.get(2).unwrap();
    let fertalizer_map = all_maps.get(1).unwrap();
    let soil_map = all_maps.get(0).unwrap();
    let soil = transform_by_map(seed, soil_map);
    let fertalizer = transform_by_map(soil, fertalizer_map);
    let water = transform_by_map(fertalizer, water_map);
    let light = transform_by_map(water, light_map);
    let temp = transform_by_map(light, temperature_map);
    let humidity = transform_by_map(temp, humidity_map);
    transform_by_map(humidity, location_map)
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
            str.retain(|c| !c.is_ascii_alphabetic());

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
        eprintln!("s {soil}");
        let fertalizer = transform_by_map(soil, fertalizer_map.as_ref().unwrap());
        eprintln!("f {fertalizer}");
        let water = transform_by_map(fertalizer, water_map.as_ref().unwrap());
        eprintln!("w {water}");
        let light = transform_by_map(water, light_map.as_ref().unwrap());
        eprintln!("l {light}");
        let temp = transform_by_map(light, temperature_map.as_ref().unwrap());
        eprintln!("t {temp}");
        let humidity = transform_by_map(temp, humidity_map.as_ref().unwrap());
        eprintln!("h {humidity}");
        let location = transform_by_map(humidity, location_map.as_ref().unwrap());
        eprintln!("lo {location}");
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
        let range = m.source..(m.source + m.range);
        if range.contains(&val) {
            let x = val - m.source;
            dest = Some(m.dest + x);
        }
    }
    if dest.is_none() {
        // No match => 1:1
        dest = Some(val);
    }

    dest.unwrap()
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
