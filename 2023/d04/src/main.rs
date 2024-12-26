use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct Card {
    id: usize,
    winning_numbers_count: usize,
}

impl Card {
    fn get_score(&self) -> usize {
        if self.winning_numbers_count <= 1 {
            return self.winning_numbers_count;
        }
        2usize.pow((self.winning_numbers_count - 1) as u32)
    }
    fn from_str(input: &str) -> Option<Self> {
        if let Some((info, numbers)) = input.split_once(":") {
            if let Some((card_str, id_str)) = info.split_once(" ") {
                if card_str == "Card" {
                    if let Ok(id) = id_str.trim().parse::<usize>() {
                        let mut winning_numbers = HashSet::new();
                        let mut card = Card {
                            id,
                            winning_numbers_count: 0,
                        };
                        if let Some((winning, mine)) = numbers.split_once("|") {
                            for winning_str in winning.split(" ") {
                                if let Ok(winning_no) = winning_str.parse::<usize>() {
                                    winning_numbers.insert(winning_no);
                                }
                            }
                            for my_str in mine.split(" ") {
                                if let Ok(my_no) = my_str.parse::<usize>() {
                                    if winning_numbers.contains(&my_no) {
                                        card.winning_numbers_count += 1;
                                    }
                                }
                            }
                        }
                        return Some(card);
                    }
                }
            }
        }
        None
    }
}

pub fn part1(input: String) -> usize {
    let mut points = 0;
    for line in input.split("\n") {
        if let Some(card) = Card::from_str(line) {
            points += card.get_score();
        }
    }
    points
}

pub fn part2(input: String) -> usize {
    let mut cards = HashMap::new();

    for line in input.split("\n") {
        if let Some(card) = Card::from_str(line) {
            cards.insert(card.id, card);
        }
    }

    let mut cards_to_add = cards.values().collect::<Vec<&Card>>();
    let mut total_card_count: usize = cards_to_add.len();

    while let Some(card) = cards_to_add.pop() {
        for i in card.id + 1..=card.id + card.winning_numbers_count {
            if let Some(card_won) = cards.get(&i) {
                total_card_count += 1;
                cards_to_add.push(card_won);
            };
        }
    }

    total_card_count
}

fn main() {}