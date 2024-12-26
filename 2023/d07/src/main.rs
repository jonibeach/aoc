use std::{cmp::Ordering, collections::HashMap};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Card {
    A,
    K,
    Q,
    Joker,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl Card {
    fn from_char(input: char) -> Option<Self> {
        match input {
            'A' => Some(Self::A),
            'K' => Some(Self::K),
            'Q' => Some(Self::Q),
            'J' => Some(Self::Joker),
            'T' => Some(Self::T),
            '9' => Some(Self::Nine),
            '8' => Some(Self::Eight),
            '7' => Some(Self::Seven),
            '6' => Some(Self::Six),
            '5' => Some(Self::Five),
            '4' => Some(Self::Four),
            '3' => Some(Self::Three),
            '2' => Some(Self::Two),
            _ => None,
        }
    }
    fn strength(&self) -> usize {
        match self {
            Self::A => 13,
            Self::K => 12,
            Self::Q => 11,
            Self::Joker => 0,
            Self::T => 9,
            Self::Nine => 8,
            Self::Eight => 7,
            Self::Seven => 6,
            Self::Six => 5,
            Self::Five => 4,
            Self::Four => 3,
            Self::Three => 2,
            Self::Two => 1,
        }
    }
}

type HandCards = [Option<Card>; 5];

#[derive(Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAkind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug)]
struct Hand {
    cards: HandCards,
    kind: HandType,
}

impl Hand {
    fn stronger_than(&self, other: &Self) -> bool {
        if self.rank() > other.rank() {
            return true;
        }
        if other.rank() > self.rank() {
            return false;
        }

        for i in 0..=4 {
            if self.cards[i].unwrap().strength() > other.cards[i].unwrap().strength() {
                return true;
            }
            if other.cards[i].unwrap().strength() > self.cards[i].unwrap().strength() {
                return false;
            }
        }
        unreachable!()
    }
    fn rank(&self) -> usize {
        match self.kind {
            HandType::HighCard => 1,
            HandType::OnePair => 2,
            HandType::TwoPair => 3,
            HandType::ThreeOfAkind => 4,
            HandType::FullHouse => 5,
            HandType::FourOfAKind => 6,
            HandType::FiveOfAKind => 7,
        }
    }

    fn from_str(input: &str) -> Option<Self> {
        let mut cards_matcher: [Option<(Card, usize)>; 5] = [None; 5];
        let mut cards: [Option<Card>; 5] = [None; 5];
        let mut card_counts = HashMap::new();
        for (i, c) in input.chars().enumerate() {
            if let Some(card) = Card::from_char(c) {
                cards[i] = Some(card);
                if let Some(existing) = card_counts.get_mut(&card) {
                    *existing += 1;
                } else {
                    card_counts.insert(card, 1usize);
                }
            }
        }

        // BTreeMap instead of HashMap so automatically sort??
        let mut card_counts_sorted = card_counts
            .iter()
            .map(|(card, count)| (card.clone(), *count))
            .collect::<Vec<(Card, usize)>>();
        card_counts_sorted.sort_by(|(_, a), (_, b)| b.cmp(a));

        for (i, (card, count)) in card_counts_sorted.iter().enumerate() {
            cards_matcher[i] = Some((card.clone(), *count))
        }
        match cards_matcher {
            [Some((_, 5)), _, _, _, _] => Some(Self {
                cards,
                kind: HandType::FiveOfAKind,
            }),
            [Some((_, 4)), Some(_), _, _, _] => Some(Self {
                cards,
                kind: HandType::FourOfAKind,
            }),
            [Some((_, 3)), Some((_, 2)), _, _, _] => Some(Self {
                cards,
                kind: HandType::FullHouse,
            }),
            [Some((_, 3)), Some((_, 1)), Some(_), _, _] => Some(Self {
                cards,
                kind: HandType::ThreeOfAkind,
            }),
            [Some((_, 2)), Some((_, 2)), Some(_), _, _] => Some(Self {
                cards,
                kind: HandType::TwoPair,
            }),
            [Some((_, 2)), Some((_, 1)), Some((_, 1)), Some((_, 1)), _] => Some(Self {
                cards,
                kind: HandType::OnePair,
            }),
            [Some(_), Some(_), Some(_), Some(_), Some(_)] => Some(Self {
                cards,
                kind: HandType::HighCard,
            }),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct HandBid {
    hand: Hand,
    bid: usize,
}

impl HandBid {
    fn from_str(input: &str) -> Option<Self> {
        if let Some((hand, bid)) = input.split_once(" ") {
            if let Some(hand) = Hand::from_str(hand) {
                if let Ok(bid) = bid.parse::<usize>() {
                    return Some(Self { hand, bid });
                }
            }
        }
        None
    }
}

pub fn part1(input: String) -> usize {
    let mut total = 0;

    let mut hand_bids = Vec::new();

    for line in input.split("\n") {
        if let Some(hand_bid) = HandBid::from_str(line) {
            hand_bids.push(hand_bid)
        }
    }

    hand_bids.sort_by(|a, b| {
        if a.hand.stronger_than(&b.hand) {
            return Ordering::Greater;
        }
        Ordering::Less
    });

    for (rank, hand_bid) in hand_bids.iter().enumerate() {
        total += (rank + 1) * hand_bid.bid;
    }

    total
}

fn main() {}