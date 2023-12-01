use aoc_23::{find_ascii_numbers, find_string_numbers};
/// Part Two here is as straight forward as part one BUT there is one thing which I did not consider
/// cases like: "twone" is two and one
/// I search for the number pattern with "find". If I found a string encoded number I would split the string at the end
/// of the found number. So in this case the resulting string is "ne" ("twone" cut after "two") and I would not find the
/// "one" in the string. This case is very rare so I had to brute force alot of lines in Unit Tests to find that I was missing this pattern
/// at a line like "eightjzqzhrllg1oneightfck" finally clicked and I saw the "oneight"
fn main() {
    let file = std::fs::read_to_string("./src/bin/day1b/input.txt").unwrap();
    let lines = file.lines();
    let mut sum = 0;
    for line in lines {
        let add = calc_line_number(line);
        sum += add;
    }
    // knowns result
    assert!(sum == 53268);
}
fn calc_line_number(s: &str) -> usize {
    // let line = lines.next().unwrap();
    // find alle ascii numbers and their index
    let ascii_numbs = find_ascii_numbers(s);
    // find all string numbers
    let string_numbers = find_string_numbers(s);

    // Combine them as they occur
    combine_digits(ascii_numbs, string_numbers)
}

fn combine_digits(ascii_numbs: Vec<(usize, char)>, string_numbers: Vec<(usize, char)>) -> usize {
    let first;
    if ascii_numbs.first().is_some() {
        if string_numbers.first().is_some() {
            if ascii_numbs.first().unwrap().0 < string_numbers.first().unwrap().0 {
                // first digit is ascci
                first = ascii_numbs.first().unwrap().1;
            } else {
                first = string_numbers.first().unwrap().1;
            }
        } else {
            first = ascii_numbs.first().unwrap().1;
        }
    } else {
        first = string_numbers.first().unwrap().1;
    }
    let last;
    if ascii_numbs.last().is_some() {
        if string_numbers.last().is_some() {
            if ascii_numbs.last().unwrap().0 > string_numbers.last().unwrap().0 {
                last = ascii_numbs.last().unwrap().1;
            } else {
                last = string_numbers.last().unwrap().1;
            }
        } else {
            last = ascii_numbs.last().unwrap().1;
        }
    } else {
        last = string_numbers.last().unwrap().1;
    }
    let mut combined_string = String::new();
    combined_string.push(first);
    combined_string.push(last);
    let add = combined_string.parse::<usize>().unwrap();
    add
}

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! test {
        ($s:expr, $num:expr) => {
            let line = $s;
            assert_eq!(calc_line_number(line), $num);
        };
    }

    #[test]
    fn test_lines() {
        // Brute force tests to find weakness in the algorithm
        let line = "kdkjqdkvgs2";
        assert_eq!(calc_line_number(line), 22);
        let line = "6threeeightjzcgsnclfive7txvgsdxnt";
        assert_eq!(calc_line_number(line), 67);
        let line = "8ninesix8monehbvmvrvsrvsqmhmxz";
        assert_eq!(calc_line_number(line), 81);
        let line = "fivexntprmkhpronejbnbseighttfnzmkdn3six";
        assert_eq!(calc_line_number(line), 56);
        let line = "nxnrsdsixeightgfbmmhhst2251eight";
        assert_eq!(calc_line_number(line), 68);
        let line = "nine339ksh9seven";
        assert_eq!(calc_line_number(line), 97);
        let line = "sixonesixjblvfqvftxpjznf5fivefour";
        assert_eq!(calc_line_number(line), 64);
        let line = "fvnvvp6twofive25twoneg";
        assert_eq!(calc_line_number(line), 61);
        let line = "fourthreebr8gnkmrh6llrlg2four";
        assert_eq!(calc_line_number(line), 44);
        let line = "kjjgzq25four19";
        assert_eq!(calc_line_number(line), 29);
        let line = "zlsfbnfkm64five24jtqmvgjtrzggqnfive";
        assert_eq!(calc_line_number(line), 65);
        let line = "8nkrfltkeight";
        assert_eq!(calc_line_number(line), 88);

        let line = "3three8dxffkzfjsevensevenseveneight";
        assert_eq!(calc_line_number(line), 38);

        test!("6threebrcrtxhgsixbrhlhnczpdbmfdgpxbcone", 61);
        test!("onexglmkkgc4", 14);
        test!("nvzeightwojjjsftdgv56fiveeightnqpqr", 88);
        test!("ninecxgvsdmmlpfxhhzfcv2threethree7rsxpdnd", 97);
        test!("xjnrxxnpxmx3", 33);
        test!("twopkhshrlksevensjzlfhpfgqkfgrnq7", 27);
        test!("6ktvsttwothreethreetktfgnnqlrtdxjjnh6", 66);
        test!("884kfive7ninethree", 83);

        test!("14xzgsbvrleightfourseven3", 13);
        test!("hndmzcqfour75zsxrlfourthreenine", 49);
        test!("five8threesqqzlfvl1", 51);
        test!("fivebsix81two9xmjq", 59);
        test!("onecgfhxvndbfqkcsbbksix2mbszjp95", 15);
        test!("5bfhqfbqsmjndthreenxlzfhlhz2", 52);
        test!("pdqkjrbxs69bgpm8six8", 68);
        test!("6onecvgfshgcnznines", 69);
        test!("twonineglpjqmkbdc7kkcz5five", 25);
        test!("lmdjlpxbg777ggpftspzjlmh118", 78);
        test!("vvblfthreeeight2msjkqjjtnfpkgqgkcxthh8", 38);
        test!("vtzdkqxksthree9fourfive", 35);
        test!("dfvlpdknmlqqf3cllbnzzbp8nlfpbqtfive", 35);
        test!("5seventwoonextsmpeighthjlcssevenone", 51);
        test!("ninezgstkmnmzfzmglcfour981", 91);

        test!("two1nine", 29);
        test!("eightwothree", 83);
        test!("abcone2threexyz", 13);
        test!("xtwone3four", 24);
        test!("4nineeightseven2", 42);
        test!("zoneight234", 14);
        test!("7pqrstsixteen", 76);

        test!("eightjzqzhrllg1oneightfck", 88);
    }

    #[test]
    fn example() {
        // Show that we produce the correct result from the given example
        let file = std::fs::read_to_string("./src/bin/day1b/test.txt").unwrap();
        let lines = file.lines();
        let mut sum = 0;
        for line in lines {
            let add = calc_line_number(line);
            sum += add;
        }
        assert!(sum == 281);
    }
}
