use aoc_23::find_ascii_numbers;
fn main() {
    let file = std::fs::read_to_string("./src/bin/day1a/input.txt").unwrap();
    let lines = file.lines();
    let mut sum: usize = 0;
    for line in lines {
        let res = find_ascii_numbers(line);
        let first = res.first();
        let last = res.last();
        if first.is_some() && last.is_some() {
            if let (Some(f), Some(l)) = (first, last) {
                let mut combined_string = String::new();
                combined_string.push(f.1);
                combined_string.push(l.1);
                sum += combined_string.parse::<usize>().unwrap();
                println!("Sum is {}", sum);
            }
        } else {
            panic!()
        }
    }
    println!("Final Sum is {}", sum);
}
