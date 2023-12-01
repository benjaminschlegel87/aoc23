pub fn find_ascii_numbers(s: &str) -> Vec<(usize, char)> {
    let mut numbers = Vec::new();
    for (i, char) in s.chars().enumerate() {
        if char.is_ascii_digit() {
            numbers.push((i, char));
        }
    }
    numbers
}
