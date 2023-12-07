fn main() {
    let file = std::fs::read_to_string("./src/bin/day7/input.txt").unwrap();

    let hands = parse_hands(&file);
    let sorted = sort_hands(hands);
    println!("Result {}", sorted.calc_total_winnings());
}

#[cfg(test)]
const EXAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
enum Label {
    A = 14,
    K = 13,
    Q = 12,
    J = 11,
    T = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
}

impl From<char> for Label {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => panic!(),
        }
    }
}
#[derive(Debug, PartialEq, Clone, Copy)]
struct Hand {
    bid: usize,
    label: [Label; 5],
    hand_type: HandType,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn check_hand_type(hand: &[Label; 5]) -> HandType {
    let mut label_cnt: Vec<(i32, Label)> = Vec::new();
    for label in hand {
        let mut cnt = 0;
        for other in hand {
            if other == label {
                cnt += 1;
            }
        }

        if cnt > 1 {
            // Check if the label was already found
            if label_cnt
                .iter()
                .find(|&&found_label| found_label.1 == *label)
                .is_none()
            {
                // At Least a pair
                label_cnt.push((cnt - 1, *label));
            }
        }
    }
    match label_cnt.len() {
        // Vector empty - Didnt found any pairs
        0 => HandType::HighCard,
        // Found atleast two cards of the same label - two ore more
        1 => {
            let (cnt, label) = label_cnt.pop().unwrap();
            match cnt {
                1 => HandType::OnePair,
                2 => HandType::ThreeOfAKind,
                3 => HandType::FourOfAKind,
                4 => HandType::FiveOfAKind,
                _ => panic!(),
            }
        }
        // Found atleast for two labels more than two cards - FullHouse or TwoPair
        2 => {
            let (cnt_1, label_1) = label_cnt.pop().unwrap();
            let (cnt_2, label_2) = label_cnt.pop().unwrap();
            if cnt_1 > 1 || cnt_2 > 1 {
                HandType::FullHouse
            } else {
                HandType::TwoPair
            }
        }
        _ => panic!("cant have more than two pairs in 5 card"),
    }
}

fn parse_hands(all_hands: &str) -> Vec<Hand> {
    let mut hands = Vec::new();
    for line in all_hands.lines() {
        let mut pair = line.split(' ');
        let labels = pair.next().unwrap();
        let bid = pair.next().unwrap();

        let mut label: Vec<Label> = Vec::new();
        for c in labels.chars() {
            label.push(c.into());
        }
        let hand_type = check_hand_type(&label.clone().try_into().unwrap());
        hands.push(Hand {
            bid: bid.parse::<usize>().unwrap(),
            label: label.try_into().unwrap(),
            hand_type,
        })
    }
    hands
}
struct SortedHands {
    hands: Vec<Hand>,
}

impl SortedHands {
    fn calc_total_winnings(&self) -> usize {
        let mut total = 0;
        for (i, hand) in self.hands.iter().enumerate() {
            total += hand.bid * (i + 1);
        }
        total
    }
}
fn sort_hands(mut hands: Vec<Hand>) -> SortedHands {
    let len = hands.len();
    let mut sorted_hands = Vec::new();
    for _i in 0..len {
        let mut lowest = Hand {
            bid: 0,
            label: [Label::A, Label::A, Label::A, Label::A, Label::A],
            hand_type: HandType::FiveOfAKind,
        };
        let mut lowest_index = len;
        for (index, hand) in hands.iter_mut().enumerate() {
            if hand.hand_type < lowest.hand_type {
                lowest = *hand;
                lowest_index = index;
            } else if hand.hand_type == lowest.hand_type {
                for (label, highest_label) in lowest.label.into_iter().zip(hand.label.into_iter()) {
                    if label > highest_label {
                        lowest = *hand;
                        lowest_index = index;
                        break;
                    } else if label < highest_label {
                        break;
                    } else {
                        // equal continue
                    }
                }
            }
        }
        hands.remove(lowest_index);
        sorted_hands.push(lowest);
    }
    SortedHands {
        hands: sorted_hands,
    }
}
#[cfg(test)]
const HIGH_CARD_EXAMPLE: &str = "23456 56";

#[cfg(test)]
const ONE_PAIR_EXAMPLE: &str = "2AA56 56";

#[cfg(test)]
const TWO_PAIR_EXAMPLE: &str = "2AA52 56";

#[cfg(test)]
const THREE_OF_A_KIND_EXAMPLE: &str = "2A252 56";

#[cfg(test)]
const FULL_HOUSE_EXAMPLE: &str = "2A2A2 56";

#[cfg(test)]
const FOUR_OF_A_KIND_EXAMPLE: &str = "AA2AA 56";

#[cfg(test)]
const FIVE_OF_A_KIND_EXAMPLE: &str = "22222 56";
#[test]
fn test_check_hand_type() {
    let hand = parse_hands(HIGH_CARD_EXAMPLE).pop().unwrap();
    assert_eq!(HandType::HighCard, check_hand_type(&hand.label));

    let hand = parse_hands(ONE_PAIR_EXAMPLE).pop().unwrap();
    assert_eq!(HandType::OnePair, check_hand_type(&hand.label));

    let hand = parse_hands(TWO_PAIR_EXAMPLE).pop().unwrap();
    assert_eq!(HandType::TwoPair, check_hand_type(&hand.label));

    let hand = parse_hands(THREE_OF_A_KIND_EXAMPLE).pop().unwrap();
    assert_eq!(HandType::ThreeOfAKind, check_hand_type(&hand.label));

    let hand = parse_hands(FULL_HOUSE_EXAMPLE).pop().unwrap();
    assert_eq!(HandType::FullHouse, check_hand_type(&hand.label));

    let hand = parse_hands(FOUR_OF_A_KIND_EXAMPLE).pop().unwrap();
    assert_eq!(HandType::FourOfAKind, check_hand_type(&hand.label));

    let hand = parse_hands(FIVE_OF_A_KIND_EXAMPLE).pop().unwrap();
    assert_eq!(HandType::FiveOfAKind, check_hand_type(&hand.label));
}
#[test]
fn test_parse_hands() {
    let mut hands = parse_hands(EXAMPLE);
    assert_eq!(
        hands.pop().unwrap(),
        Hand {
            bid: 483,
            label: [Label::Q, Label::Q, Label::Q, Label::J, Label::A,],
            hand_type: HandType::ThreeOfAKind,
        }
    );

    assert_eq!(
        hands.pop().unwrap(),
        Hand {
            bid: 220,
            label: [Label::K, Label::T, Label::J, Label::J, Label::T,],
            hand_type: HandType::TwoPair,
        }
    );

    assert_eq!(
        hands.pop().unwrap(),
        Hand {
            bid: 28,
            label: [Label::K, Label::K, Label::Six, Label::Seven, Label::Seven,],
            hand_type: HandType::TwoPair,
        }
    );

    assert_eq!(
        hands.pop().unwrap(),
        Hand {
            bid: 684,
            label: [Label::T, Label::Five, Label::Five, Label::J, Label::Five,],
            hand_type: HandType::ThreeOfAKind,
        }
    );

    assert_eq!(
        hands.pop().unwrap(),
        Hand {
            bid: 765,
            label: [Label::Three, Label::Two, Label::T, Label::Three, Label::K,],
            hand_type: HandType::OnePair,
        }
    );
}

#[test]
fn test_sorted_hands() {
    let hands = parse_hands(EXAMPLE);
    let sorted = sort_hands(hands);
    for s in &sorted.hands {
        eprintln!("{:?}", s);
    }
    assert_eq!(6440, sorted.calc_total_winnings());
}
