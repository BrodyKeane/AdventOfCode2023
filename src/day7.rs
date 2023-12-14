use std::{usize, fs, collections::HashMap,};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug)]
enum Card {
    Unknown,
    Joker,
    Number(u8),
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            'A' | 'a' => Card::Ace,
            'K' | 'k' => Card::King,
            'Q' | 'q' => Card::Queen,
            'J' | 'j' => Card::Joker,
            '2'..='9' => {
                let num = c.to_digit(10).unwrap() as u8;
                Card::Number(num)
            }
            'T' => Card::Number(10),
            unknown => panic!("Invalid character for card: {}", unknown),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    hand_type: HandType,
    cards: Vec<Card>,
    bid: usize,
}

pub fn total_winnings(path: &str) -> usize {
    let mut hands: Vec<Hand> = fs::read_to_string(path)
        .expect(&format!("Failed to read: {}", path))
        .lines()
        .map(|line| {
            let mut strs = line.split_ascii_whitespace();
            Hand::new(strs.next().unwrap(), strs.next().unwrap().parse().unwrap())
        })
        .collect::<Vec<Hand>>();

    hands.sort();

    hands.into_iter()
        .enumerate()
        .fold(0, |acc, (rank, hand)| hand.bid * (rank + 1) + acc)
}

impl Hand {
    fn new(card_str: &str, bid: usize) -> Self {
        let cards = Self::find_cards(card_str);
        let hand_type = Self::find_hand_type(&cards);
        Self { hand_type, cards, bid } 
    }

    fn find_hand_type(cards: &Vec<Card>) -> HandType {
        let mut card_counts: HashMap<Card, usize> = HashMap::new();
        for card in cards {
            match card_counts.get(&card) {
                Some(count) => card_counts.insert(*card, count + 1),
                None => card_counts.insert(*card, 1),
            };
        }

        let mut max: usize = 0;
        let mut pairs = 0;

        for (card, count) in card_counts.iter() {

            if max < *count && *card != Card::Joker {
                max = *count;
            }

            if *count >= 2 && *card != Card::Joker {
                pairs += 1;
            }
        }

        let joker_count = card_counts.get(&Card::Joker).unwrap_or(&0);

        if *joker_count > 0 && max < 2 {
            pairs += 1;
        }
        max += *joker_count;



        match (max, pairs) {
            (5, 1) => HandType::FiveOfAKind,
            (4, 1) => HandType::FourOfAKind,
            (3, 2) => HandType::FullHouse,
            (3, 1) => HandType::ThreeOfAKind,
            (2, 2) => HandType::TwoPair,
            (2, 1) => HandType::OnePair,
            (_, _) => HandType::HighCard,
        }
    }

    fn find_cards(card_str: &str) -> Vec<Card> {
        card_str.chars()
            .map(|c| Card::from(c))
            .collect::<Vec<Card>>()
    }
}


