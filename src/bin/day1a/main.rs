use aoc_23::find_ascii_numbers;
/// Finding ASCII Numeric chars is very easy in Rust as shown in [find_ascii_numbers]
/// From here its only cominbing the first and last element of the vector to the combined number
/// this string number can then be parsed in a usize
/// add all up - thats it
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
            }
        } else {
            panic!()
        }
    }
    // After doing it I know now this is the answer - Keep assert for future changes in lib
    assert!(53080 == sum);
}
