fn main() {
    let file = std::fs::read_to_string("./src/bin/day3/input.txt").unwrap();

    let input = file;
    let symbols = AllSymbols::new(&input);
    let mut numbers = Vec::new();
    for (i, s) in input.lines().enumerate() {
        numbers.push(extract_numbers_from_line(s, i + 1));
    }
    let mut sum = 0;
    for nums in &numbers {
        for n in nums {
            for coords in &n.symbol_pos {
                // check for this number if any possible position is in the symbol position
                for symbol in &symbols.symbols {
                    if symbol.coord == *coords {
                        //Add
                        sum += n.number;
                        break;
                    }
                }
            }
        }
    }
    println!("{sum}");
    assert!(sum == 540131);

    let mut total_gear = 0;
    for symbol in &symbols.symbols {
        if symbol.symbol == '*' {
            let mut cnt = 0;
            let mut gear_score = 0;
            for nums in &numbers {
                for n in nums {
                    for coords in &n.symbol_pos {
                        if *coords == symbol.coord {
                            cnt += 1;
                            if cnt == 1 {
                                gear_score = n.number;
                            }
                            if cnt == 2 {
                                gear_score *= n.number;
                            }
                            if cnt > 2 {
                                gear_score = 0;
                            }
                        }
                    }
                }
            }
            if cnt == 1 {
                gear_score = 0;
            }
            total_gear += gear_score;
        }
    }
    println!("{total_gear}");
    assert!(total_gear == 86879020);
}

// jeder Reihe hat eine konstante Anzahl
// Es gibt ASCII Nummern, Punkte oder Symbole
// alles was kein Punkt und kein ASCII Symbol ist ist ein Symbol
// Finde alle Symbole und gebe ihnen einen x/y Wert Reihe/Position
// Finde alle Zahlen und mache ermittle dabei alle validen Symbol Koordinaten
// added die Zahl nur wenn sich ein Symbol in den validen Koordinaten befindet
#[cfg(test)]
const EXAMPLE: &str = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
// Impl == for SymbolCoordina<QAte so its easy to compare if it is a much
impl Eq for SymbolCoordinate {}
#[derive(PartialEq, Debug)]
struct SymbolCoordinate {
    row: usize,
    pos: usize,
}
impl SymbolCoordinate {
    pub fn new(row: usize, pos: usize) -> Self {
        Self { row, pos }
    }
}
#[derive(Debug, PartialEq)]
struct Symbol {
    symbol: char,
    coord: SymbolCoordinate,
}
impl Symbol {
    #[cfg(test)]
    pub fn new(c: char, row: usize, pos: usize) -> Self {
        Self {
            symbol: c,
            coord: SymbolCoordinate { row, pos },
        }
    }
}
struct Numbers {
    number: usize,
    symbol_pos: Vec<SymbolCoordinate>,
}
fn extract_numbers_from_line(s: &str, row: usize) -> Vec<Numbers> {
    let mut result = Vec::new();
    let mut remain = s;
    let mut old_index = 0;
    while !remain.is_empty() {
        let num = extract_number(remain);
        if let Some(number) = num {
            // found number
            let index = old_index + number.1;
            let split = remain.split_at(number.1 + number.0.len());
            remain = split.1;

            let min_index = if index > 0 { index - 1 } else { index };
            let max_index = index + number.0.len();
            // min index | Number | max index => lower an upper bounds
            result.push(Numbers {
                number: number.0.parse::<usize>().unwrap(),
                symbol_pos: build_possible_coordinates(min_index, max_index, row),
            });
            old_index += split.0.len();
        } else {
            // No new numbers
            break;
        }
    }
    result
}
fn extract_number(s: &str) -> Option<(String, usize)> {
    let mut numb = String::new();
    let mut start_index = 0;
    for (i, c) in s.chars().enumerate() {
        if c.is_numeric() {
            if numb.is_empty() {
                start_index = i;
            }
            numb.push(c);
        } else if !numb.is_empty() {
            break;
        }
    }
    if !numb.is_empty() {
        Some((numb, start_index))
    } else {
        None
    }
}

fn build_possible_coordinates(
    min_index: usize,
    max_index: usize,
    row: usize,
) -> Vec<SymbolCoordinate> {
    let mut res = Vec::new();
    res.push(SymbolCoordinate::new(row, min_index));
    res.push(SymbolCoordinate::new(row, max_index));
    for i in min_index..=max_index {
        res.push(SymbolCoordinate {
            row: row - 1,
            pos: i,
        });
        res.push(SymbolCoordinate {
            row: row + 1,
            pos: i,
        });
    }
    res
}

#[derive(Debug)]
struct AllSymbols {
    symbols: Vec<Symbol>,
}
impl AllSymbols {
    pub fn new(s: &str) -> Self {
        let mut symbols = AllSymbols {
            symbols: Vec::new(),
        };
        for (i, line) in s.lines().enumerate() {
            for (n, c) in line.chars().enumerate() {
                if c != '.' && !c.is_numeric() {
                    symbols.symbols.push(Symbol {
                        symbol: c,
                        coord: SymbolCoordinate { row: i + 1, pos: n },
                    });
                }
            }
        }
        symbols
    }
}
#[test]
fn extract_symbols() {
    let sym = AllSymbols::new(EXAMPLE);
    assert_eq!(*sym.symbols.get(0).unwrap(), Symbol::new('*', 3, 3));

    assert_eq!(*sym.symbols.get(1).unwrap(), Symbol::new('#', 5, 6));

    assert_eq!(*sym.symbols.get(2).unwrap(), Symbol::new('*', 6, 3));
    assert_eq!(*sym.symbols.get(3).unwrap(), Symbol::new('+', 7, 5));
    assert_eq!(*sym.symbols.get(4).unwrap(), Symbol::new('$', 10, 3));
    assert_eq!(*sym.symbols.get(5).unwrap(), Symbol::new('*', 10, 5));

    assert!(sym.symbols.get(6).is_none())
}

#[test]
fn test_extract_numbers() {
    let s = extract_number("..123..").unwrap();
    assert!(s.1 == 2);
    assert_eq!(s.0, String::from("123"));
}
#[test]
fn test_build_possible_coords() {
    // 0
    // 1
    // 2
    let mut coor = build_possible_coordinates(2, 5, 1);
    assert_eq!(coor.pop().unwrap(), SymbolCoordinate::new(2, 5));
    assert_eq!(coor.pop().unwrap(), SymbolCoordinate::new(0, 5));
    assert_eq!(coor.pop().unwrap(), SymbolCoordinate::new(2, 4));
    assert_eq!(coor.pop().unwrap(), SymbolCoordinate::new(0, 4));
    assert_eq!(coor.pop().unwrap(), SymbolCoordinate::new(2, 3));
    assert_eq!(coor.pop().unwrap(), SymbolCoordinate::new(0, 3));
    assert_eq!(coor.pop().unwrap(), SymbolCoordinate::new(2, 2));
    assert_eq!(coor.pop().unwrap(), SymbolCoordinate::new(0, 2));
    assert_eq!(coor.pop().unwrap(), SymbolCoordinate::new(1, 5));
    assert_eq!(coor.pop().unwrap(), SymbolCoordinate::new(1, 2));
}

#[test]
fn test_extract_numbers_from_line() {
    let s = "467..114..";
    let mut n = extract_numbers_from_line(s, 1);
    assert!(n.len() == 2);

    let mut number = n.pop().unwrap();
    assert_eq!(number.number, 114);
    assert_eq!(
        number.symbol_pos.pop().unwrap(),
        SymbolCoordinate::new(2, 8)
    );
    assert_eq!(
        number.symbol_pos.pop().unwrap(),
        SymbolCoordinate::new(0, 8)
    );
    assert_eq!(
        number.symbol_pos.pop().unwrap(),
        SymbolCoordinate::new(2, 7)
    );
    assert_eq!(
        number.symbol_pos.pop().unwrap(),
        SymbolCoordinate::new(0, 7)
    );
    assert_eq!(
        number.symbol_pos.pop().unwrap(),
        SymbolCoordinate::new(2, 6)
    );
    assert_eq!(
        number.symbol_pos.pop().unwrap(),
        SymbolCoordinate::new(0, 6)
    );

    assert_eq!(
        number.symbol_pos.pop().unwrap(),
        SymbolCoordinate::new(2, 5)
    );
    assert_eq!(
        number.symbol_pos.pop().unwrap(),
        SymbolCoordinate::new(0, 5)
    );
    assert_eq!(
        number.symbol_pos.pop().unwrap(),
        SymbolCoordinate::new(2, 4)
    );
    assert_eq!(
        number.symbol_pos.pop().unwrap(),
        SymbolCoordinate::new(0, 4)
    );
    assert_eq!(
        number.symbol_pos.pop().unwrap(),
        SymbolCoordinate::new(1, 8)
    );
    assert_eq!(
        number.symbol_pos.pop().unwrap(),
        SymbolCoordinate::new(1, 4)
    );

    let mut number = n.pop().unwrap();
    assert_eq!(number.number, 467);
    assert_eq!(
        number.symbol_pos.pop().unwrap(),
        SymbolCoordinate::new(2, 3)
    );
    assert_eq!(
        number.symbol_pos.pop().unwrap(),
        SymbolCoordinate::new(0, 3)
    );
    assert_eq!(
        number.symbol_pos.pop().unwrap(),
        SymbolCoordinate::new(2, 2)
    );
    assert_eq!(
        number.symbol_pos.pop().unwrap(),
        SymbolCoordinate::new(0, 2)
    );
    assert_eq!(
        number.symbol_pos.pop().unwrap(),
        SymbolCoordinate::new(2, 1)
    );
    assert_eq!(
        number.symbol_pos.pop().unwrap(),
        SymbolCoordinate::new(0, 1)
    );

    assert_eq!(
        number.symbol_pos.pop().unwrap(),
        SymbolCoordinate::new(2, 0)
    );
    assert_eq!(
        number.symbol_pos.pop().unwrap(),
        SymbolCoordinate::new(0, 0)
    );
    assert_eq!(
        number.symbol_pos.pop().unwrap(),
        SymbolCoordinate::new(1, 3)
    );
    assert_eq!(
        number.symbol_pos.pop().unwrap(),
        SymbolCoordinate::new(1, 0)
    );
}
#[test]
fn test_all() {
    let input = ".......497...........................858...923...128..................227..801........487.....664...........................................
436........765..............140.......+....................859.............*.........+.................960........668.......................";
    let symbols = AllSymbols::new(&input);
    let mut numbers = Vec::new();
    for (i, s) in input.lines().enumerate() {
        numbers.push(extract_numbers_from_line(s, i + 1));
    }
    let mut sum = 0;
    for nums in numbers {
        for n in nums {
            for i in n.symbol_pos {
                if n.number == 801 {
                    eprintln!("Possible Positions {:?}", i);
                }
                // check for this number if any possible position is in the symbol position
                for symbol in &symbols.symbols {
                    if symbol.coord == i {
                        //Add
                        eprintln!(
                            "Found symbol matching to number: Number KOord {:?} Symbol Koord {:?}",
                            i, symbol.coord
                        );
                        sum += n.number;
                        break;
                    }
                }
            }
        }
    }
    assert_eq!(sum, 2146);
}

#[test]
fn test_all_2() {
    let input = "436........765..............140.......+....................859.............*.........+.................960........668.......................
...*982...........=..........=....203......266.263...375*....=...402....691..-....................*..........575....................13......
.............114...588...............*............*......631........*.......952...463..14.......661..........=...706......*333.........595..";

    let symbols = AllSymbols::new(&input);
    let mut numbers = Vec::new();
    for (i, s) in input.lines().enumerate() {
        numbers.push(extract_numbers_from_line(s, i + 1));
    }
    let mut sum = 0;
    for nums in numbers {
        for n in nums {
            for i in n.symbol_pos {
                if n.number == 801 {
                    eprintln!("Possible Positions {:?}", i);
                }
                // check for this number if any possible position is in the symbol position
                for symbol in &symbols.symbols {
                    if symbol.coord == i {
                        //Add
                        eprintln!(
                            "Found symbol matching to number: Number KOord {:?} Symbol Koord {:?}",
                            i, symbol.coord
                        );
                        sum += n.number;
                        break;
                    }
                }
            }
        }
    }
    assert_eq!(sum, 8091);
}
