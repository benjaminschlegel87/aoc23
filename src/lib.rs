/// Takes a string slice and returns all a vector of  all ascii numbers as single digits together with the index where it was found
///
/// # Example
/// ```
/// # use aoc_23::find_ascii_numbers;
/// let s = "This is AoC23 Day1";
/// let numbers = find_ascii_numbers(s);
/// assert_eq!(numbers.len(), 3);
/// assert_eq!(numbers.first().unwrap().1, '2');
/// assert_eq!(numbers.last().unwrap().1, '1');
/// assert_eq!(numbers.get(1).unwrap().1, '3');
///
pub fn find_ascii_numbers(s: &str) -> Vec<(usize, char)> {
    let mut numbers = Vec::new();
    for (i, char) in s.chars().enumerate() {
        if char.is_ascii_digit() {
            numbers.push((i, char));
        }
    }
    numbers
}

/// Takes a string slice and returns all a vector of  all string endcoded numbers as single digits together with the index where it was found
///
/// This works only for lower case numbers => "One" will not be found
/// # Example
/// ```
/// # use aoc_23::find_string_numbers;
/// let s = "Day one is harder than day five";
/// let numbers = find_string_numbers(s);
/// assert_eq!(numbers.len(), 2);
/// assert_eq!(numbers.first().unwrap().1, '1');
/// assert_eq!(numbers.last().unwrap().1, '5');
///
pub fn find_string_numbers(s: &str) -> Vec<(usize, char)> {
    let mut current_string = s;
    let mut string_numbers = Vec::new();
    let mut offset = 0;
    while current_string != "" {
        let num = find_number(current_string);
        if let Some((number, index)) = num {
            // we found a string number in this string part
            let new = current_string.split_at(1 + index);
            current_string = new.1;
            string_numbers.push((offset + index, number));
            offset += new.0.len();
        } else {
            // End loop if no numbers where found
            current_string = "";
        }
    }
    string_numbers
}

const ONE: &str = "one";
const TWO: &str = "two";
const THREE: &str = "three";
const FOUR: &str = "four";
const FIVE: &str = "five";
const SIX: &str = "six";
const SEVEN: &str = "seven";
const EIGHT: &str = "eight";
const NINE: &str = "nine";
const NUMBERS: [&str; 9] = [ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE];

fn find_number(s: &str) -> Option<(char, usize)> {
    let mut res = None;
    for number in NUMBERS {
        let index = s.find(number);
        if let Some(index) = index {
            if res.is_none() {
                //first number
                res = Some((get_number_from_string(number), index));
            } else {
                // second num
                if res.unwrap().1 > index {
                    res = Some((get_number_from_string(number), index));
                }
            }
        }
    }
    res
}
fn get_number_from_string(s: &str) -> char {
    match s {
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_numbers() {
        let s = "5sixfour2qxsqkpnq";
        let num = find_string_numbers(s);
        eprintln!("{:?}", num);
        assert!(num.len() == 2);
        assert_eq!(num.first().unwrap().1, '6');
        assert_eq!(num.last().unwrap().1, '4');
    }
}
