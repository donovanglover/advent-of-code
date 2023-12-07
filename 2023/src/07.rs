use indexmap::IndexMap;

#[derive(Ord, Eq, PartialEq, PartialOrd, Debug)]
struct Hand {
    hand: String,
    strength: u32,
    bid: u32,
    tiebreaker: [u32; 5]
}

static STRENGTH: [char; 13] = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];

#[derive(PartialEq, Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

static HAND_STRENGTH: [HandType; 7] = [
    HandType::FiveOfAKind,
    HandType::FourOfAKind,
    HandType::FullHouse,
    HandType::ThreeOfAKind,
    HandType::TwoPair,
    HandType::OnePair,
    HandType::HighCard,
];

pub fn camel_cards(str: &str) -> u32 {
    let mut total_winnings = 0;
    let mut hands: Vec<Hand> = vec![];

    for line in str.lines() {
        let hand: String = line.split_whitespace().next().unwrap().to_string();
        let bid: u32 = line.split_whitespace().last().unwrap().parse().unwrap();
        let mut letters: IndexMap<char, u32> = IndexMap::new();

        for ch in hand.chars() {
            *letters.entry(ch).or_insert(0) += 1;
        }

        letters.sort_by(|_, v1, _, v2| v2.cmp(v1));

        let hand_type: HandType = get_type(letters);
        let strength: u32 = get_hand_strength(hand_type);
        let tiebreaker = get_tiebreaker(&hand);

        hands.push(Hand { hand, strength, bid, tiebreaker })
    }

    hands.sort_by(|a, b| b.strength.cmp(&a.strength).then(b.tiebreaker.cmp(&a.tiebreaker)));

    hands.reverse();

    for (i, hand) in hands.iter().enumerate() {
        total_winnings += hand.bid * ((i as u32) + 1);
    }

    total_winnings
}

fn get_tiebreaker(hand: &String) -> [u32; 5] {
    let mut array: [u32; 5] = [0, 0, 0, 0, 0];

    for (i, char) in hand.chars().enumerate() {
        for j in 0..STRENGTH.len() {
            if STRENGTH[j] == char {
                array[i] = (STRENGTH.len() - j) as u32;
            }
        }
    }

    array
}

fn get_type(letters: IndexMap<char, u32>) -> HandType {
    for (_, v) in &letters {
        match v {
            5 => return HandType::FiveOfAKind,
            4 => return HandType::FourOfAKind,
            3 => {
                if letters[1] == 2 {
                    return HandType::FullHouse
                } else {
                    return HandType::ThreeOfAKind
                }
            },
            2 => {
                if letters[1] == 2 {
                    return HandType::TwoPair
                } else {
                    return HandType::OnePair
                }
            },
            1 => return HandType::HighCard,
            _ => {},
        }
    }

    HandType::HighCard
}

fn get_hand_strength(hand_type: HandType) -> u32 {
    for i in 0..HAND_STRENGTH.len() {
        if HAND_STRENGTH[i] == hand_type {
            return (HAND_STRENGTH.len() - i) as u32;
        }
    }

    0
}

#[cfg(test)]
mod part1 {
    #[test]
    fn example() {
        assert_eq!(super::camel_cards("\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"), 6440);
    }

    #[test]
    fn input() {
        assert_eq!(super::camel_cards(&sugar::input(file!())), 251136060);
    }
}
