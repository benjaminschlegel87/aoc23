fn main() {
    let file = std::fs::read_to_string("./src/bin/day4/input.txt").unwrap();
    // parse input file to type holding all relevant informations integers
    let mut cards = create_all_cards(file);
    // Calculate total worth with rules from Part 1
    let res = calc_total_worth(&cards);
    println!("{res}");
    assert!(20667 == res);
    // Calculate total card cnt with rules from Part 2
    let res = calc_total_cnt(&mut cards);
    println!("{res}");
    assert!(5833065 == res);
}

#[cfg(test)]
const EXAMPLE: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

/// Sum up worth of all cards
fn calc_total_worth(all_cards: &Vec<Card>) -> usize {
    let mut total_worth = 0;
    for cards in all_cards {
        total_worth += cards.calc_worth();
    }
    total_worth
}

/// Caculate the total cnt of all cards as the rules in part 2 dicatate
fn calc_total_cnt(all_cards: &mut Vec<Card>) -> usize {
    // Clone Vector to work around the Borrow Checker
    let mut new_cards = all_cards.clone();
    let len = all_cards.len();
    for cards in all_cards {
        // Update actual card count from copy vector
        // Iterator borrows the current cards
        // As a member of the vector is borrowed so is the whole vector
        // we cannot mutate other members of the vector
        // so increment the card cnt from other vector members we need to
        // write this increment to the copy vector new_cards
        // here we now can update the card with the cnt from the copy vector
        cards.cnt = new_cards.get(cards.card_num - 1).unwrap().cnt;

        // calculate how many winning numbers are on this card
        let winning_nr = cards.calc_winning_number();

        //  We apply the winning number to the next indicies for the cards.cnt amount of times
        // as copied cards can yield also more new cards
        for _n in 0..cards.cnt {
            // Increment the next indices cnt accoring to the winnings nr rules in Part 2
            for i in 0..winning_nr {
                let index = cards.card_num + i;
                if index < len {
                    // cards is borrwed - cannot write to the index of the vector all_cards
                    // write it to copy vector new_cards
                    new_cards.get_mut(index).unwrap().cnt += 1;
                }
            }
        }
    }
    // sum up all Cnts in the copy vector
    let mut total_cnt = 0;
    for card in new_cards {
        total_cnt += card.cnt;
    }

    total_cnt
}

fn create_all_cards(game_string: String) -> Vec<Card> {
    let mut all_cards = Vec::new();
    for line in game_string.lines() {
        all_cards.push(Card::new(CardStrings::new(line)));
    }
    all_cards
}

#[derive(Debug, Clone)]
struct Card {
    cnt: usize,
    card_num: usize,
    winners: Vec<usize>,
    numbers: Vec<usize>,
}

impl Card {
    /// Calculate worth of a card as described in Part 1
    fn calc_worth(&self) -> usize {
        let mut worth = 0;
        for n in &self.numbers {
            for win in &self.winners {
                if n == win {
                    if worth == 0 {
                        worth = 1;
                    } else {
                        worth *= 2;
                    }
                }
            }
        }
        worth
    }

    /// Calulate total number of winnings per card (number matches winning number)
    fn calc_winning_number(&self) -> usize {
        let mut winnings = 0;
        for n in &self.numbers {
            for win in &self.winners {
                if n == win {
                    winnings += 1;
                }
            }
        }
        winnings
    }
}

#[derive(Debug)]
struct CardStrings {
    card_num: String,
    winners: String,
    numbers: String,
}
impl CardStrings {
    /// Parses the Line format into separate Strings representing the three different informations: Card Number, Winner Numbers and our Numbers
    fn new(s: &str) -> Self {
        let colon = s.find(':').unwrap();
        let temp = s.split_at(colon);
        let card_number = temp.0;

        let bar = temp.1.find('|').unwrap();
        let (mut winners, mut numbers) = temp.1.split_at(bar);
        winners = winners.strip_prefix(':').unwrap();
        numbers = numbers.strip_prefix('|').unwrap();

        Self {
            card_num: String::from(card_number),
            winners: String::from(winners),
            numbers: String::from(numbers),
        }
    }
}
impl Card {
    fn new(card: CardStrings) -> Self {
        let card_num = remove_non_numeric(&card.card_num).parse::<usize>().unwrap();
        let mut winners = Vec::new();

        for win in card.winners.split(' ') {
            if !win.is_empty() {
                let win = win.trim();
                if !win.is_empty() {
                    winners.push(win.parse::<usize>().unwrap());
                }
            }
        }
        let mut numbers = Vec::new();
        for num in card.numbers.split(' ') {
            if !num.is_empty() {
                let num = num.trim();
                if !num.is_empty() {
                    numbers.push(num.parse::<usize>().unwrap());
                }
            }
        }
        Card {
            cnt: 1,
            card_num,
            winners,
            numbers,
        }
    }
}

fn remove_non_numeric(s: &str) -> String {
    let mut res = String::new();
    for c in s.chars() {
        if c.is_numeric() {
            res.push(c);
        }
    }
    res
}

#[test]
fn test_parse_card() {
    let mut cards = create_all_cards(String::from(EXAMPLE));
    assert!(13 == calc_total_worth(&cards));
    assert!(30 == calc_total_cnt(&mut cards));
}
