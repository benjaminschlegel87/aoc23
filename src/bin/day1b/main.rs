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

use core::panic;

use aoc_23::find_ascii_numbers;
// ich suche mir alle Nummern in einer Line
// einmal per .chars
// einmal über find "one" "two" etc.
// jeder Treffer speichert seinen index (startindex für string zahlen)
// danach regelt man über den Index wer erster und wer letzter ist
fn main() {
    let file = std::fs::read_to_string("./src/bin/day1b/input.txt").unwrap();
    let lines = file.lines();
    let mut sum = 0;
    let mut tot_lines = 0;
    for line in lines {
        let add = calc_line_number(line);
        println!("Add is {}", add);
        sum += add;
        println!("Sum is {}", sum);
        tot_lines += 1;
    }
    println!("Total lines {}", tot_lines);
    println!("final Sum is {}", sum);
}
fn calc_line_number(s: &str) -> usize {
    // let line = lines.next().unwrap();
    // find alle ascii numbers and their index
    let ascii_numbs = find_ascii_numbers(s);
    // find all string numbers
    let string_numbers = find_string_numbers(s);
    eprintln!("Strings {:?}", string_numbers);
    println!("Ascii {:?}", ascii_numbs);

    combine_digits(ascii_numbs, string_numbers)

    // fix these together and sum them up
}

fn combine_digits(ascii_numbs: Vec<(usize, char)>, string_numbers: Vec<(usize, &str)>) -> usize {
    let first;
    if ascii_numbs.first().is_some() {
        if string_numbers.first().is_some() {
            if ascii_numbs.first().unwrap().0 < string_numbers.first().unwrap().0 {
                // first digit is ascci
                first = ascii_numbs.first().unwrap().1;
            } else {
                if string_numbers.first().unwrap().1.len() == 1 {
                    first = string_numbers.first().unwrap().1.chars().next().unwrap();
                } else {
                    panic!();
                }
            }
        } else {
            first = ascii_numbs.first().unwrap().1;
        }
    } else {
        if string_numbers.first().unwrap().1.len() == 1 {
            first = string_numbers.first().unwrap().1.chars().next().unwrap();
        } else {
            panic!();
        }
    }
    let last;
    if ascii_numbs.last().is_some() {
        if string_numbers.last().is_some() {
            if ascii_numbs.last().unwrap().0 > string_numbers.last().unwrap().0 {
                last = ascii_numbs.last().unwrap().1;
            } else {
                if string_numbers.last().unwrap().1.len() == 1 {
                    last = string_numbers.last().unwrap().1.chars().next().unwrap();
                } else {
                    panic!();
                }
            }
        } else {
            last = ascii_numbs.last().unwrap().1;
        }
    } else {
        if string_numbers.last().unwrap().1.len() == 1 {
            last = string_numbers.last().unwrap().1.chars().next().unwrap();
        } else {
            panic!();
        }
    }
    let mut combined_string = String::new();
    combined_string.push(first);
    combined_string.push(last);
    let add = combined_string.parse::<usize>().unwrap();
    add
}
fn find_string_numbers(s: &str) -> Vec<(usize, &str)> {
    let mut current_string = s;
    let mut string_numbers = Vec::new();
    let mut offset = 0;
    while current_string != "" {
        let num = find_number(current_string);
        if let Some((len, number, index)) = num {
            // we found a string number in this string part
            let new = current_string.split_at(len + index);
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

fn find_number(s: &str) -> Option<(usize, &str, usize)> {
    let mut res = None;
    for number in NUMBERS {
        let index = s.find(number);
        if let Some(index) = index {
            if res.is_none() {
                //first number
                res = Some((
                    number.as_bytes().len(),
                    get_number_from_string(number),
                    index,
                ));
            } else {
                // second num
                if res.unwrap().2 > index {
                    res = Some((
                        number.as_bytes().len(),
                        get_number_from_string(number),
                        index,
                    ));
                }
            }
        }
    }
    res
}
fn get_number_from_string(s: &str) -> &str {
    match s {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => panic!(),
    }
}

#[test]
fn test_number_parsing() {
    assert_eq!(get_number_from_string(ONE), "1");
    assert_eq!(get_number_from_string(TWO), "2");
    assert_eq!(get_number_from_string(THREE), "3");
    assert_eq!(get_number_from_string(FOUR), "4");
    assert_eq!(get_number_from_string(FIVE), "5");
    assert_eq!(get_number_from_string(SIX), "6");
    assert_eq!(get_number_from_string(SEVEN), "7");
    assert_eq!(get_number_from_string(EIGHT), "8");
    assert_eq!(get_number_from_string(NINE), "9");
}

#[test]
fn test_split_at() {
    let s = "LONG";
    assert_eq!("", s.split_at(4).1);
}

#[test]
fn test_string_numbers() {
    let s = "5sixfour2qxsqkpnq";
    let num = find_string_numbers(s);
    eprintln!("{:?}", num);
    assert!(num.len() == 2);
    assert_eq!(num.first().unwrap().1, "6");
    assert_eq!(num.last().unwrap().1, "4");
}

macro_rules! test {
    ($s:expr, $num:expr) => {
        let line = $s;
        assert_eq!(calc_line_number(line), $num);
    };
}

#[test]
fn test_lines() {
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
    assert_eq!(calc_line_number(line), 62);
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
    let v = vec![1];
    assert_eq!(*v.first().unwrap(), 1);
    assert_eq!(*v.last().unwrap(), 1);
}
