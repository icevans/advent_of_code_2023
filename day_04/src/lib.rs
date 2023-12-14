use std::collections::HashMap;
use regex::Regex;

const CARD_DESCRIPTION_PATTERN: &str = r"Card +([0-9]+): (.+) \| (.+)";

pub struct CardPile {
    pub cards: Vec<Card>,
    pub card_counts: HashMap<u32, u32>
}

impl CardPile {
    pub fn new(cards: Vec<Card>) -> CardPile {
        let mut card_counts = HashMap::new();
        for card in cards.iter() {
            let count = card_counts.entry(card.id).or_insert(0);
            *count += 1;
        }

        CardPile {
            cards,
            card_counts,
        }
    }

    pub fn get_original_card_by_id(&self, id: u32) -> &Card {
        &self.cards[id as usize - 1]
    }

    pub fn add_copy_of_card_with_id(&mut self, id: u32) {
        let count = self.card_counts.entry(id).or_insert(0);
        *count += 1;
    }

    pub fn total_cards(&self) -> u32 {
        self.card_counts.keys().map(|key| self.card_counts[key] ).sum()
    }
}

#[derive(PartialEq, Debug)]
pub struct Card {
    pub id: u32,
    owned_numbers: Vec<u32>,
    winning_numbers: Vec<u32>,
}

impl Card {
    pub fn from(card_description: &str) -> Card {
        let card_description_regex = Regex::new(CARD_DESCRIPTION_PATTERN).expect("valid regex");
        let (_, [card_id, winning_numbers, owned_numbers]) = card_description_regex
            .captures(card_description)
            .unwrap()
            .extract::<3>();

        Card {
            id: card_id
                .parse()
                .expect("card id in description should be number"),
            winning_numbers: winning_numbers
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect(),
            owned_numbers: owned_numbers
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect(),
        }
    }

    pub fn worth(&self) -> u32 {
        let matches: Vec<&u32> = self
            .owned_numbers
            .iter()
            .filter(|owned_number| self.winning_numbers.contains(owned_number))
            .collect();

        if matches.len() == 0 {
            return 0;
        }

        let base: u32 = 2;
        base.pow(matches.len() as u32 - 1)
    }

    pub fn how_many_wins(&self) -> u32 {
        self.owned_numbers
            .iter()
            .filter(|owned_number| self.winning_numbers.contains(owned_number))
            .count() as u32
    }
}

#[cfg(test)]
mod card_tests {
    use super::*;

    #[test]
    fn card_worth_1() {
        let card = Card {
            id: 1,
            owned_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
            winning_numbers: vec![41, 48, 83, 86, 17],
        };
        assert_eq!(8, card.worth())
    }

    #[test]
    fn card_1_how_many_wins() {
        let card = Card {
            id: 1,
            owned_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
            winning_numbers: vec![41, 48, 83, 86, 17],
        };
        assert_eq!(4, card.how_many_wins())
    }

    #[test]
    fn card_description_1() {
        let card = Card {
            id: 1,
            owned_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
            winning_numbers: vec![41, 48, 83, 86, 17],
        };

        assert_eq!(
            card,
            Card::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53")
        );
    }

    #[test]
    fn card_worth_2() {
        let card = Card {
            id: 1,
            owned_numbers: vec![61, 30, 68, 82, 17, 32, 24, 19],
            winning_numbers: vec![13, 32, 20, 16, 61],
        };
        assert_eq!(2, card.worth())
    }

    #[test]
    fn card_description_2() {
        let card = Card {
            id: 2,
            owned_numbers: vec![61, 30, 68, 82, 17, 32, 24, 19],
            winning_numbers: vec![13, 32, 20, 16, 61],
        };
        assert_eq!(
            card,
            Card::from("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19")
        )
    }

    #[test]
    fn card_worth_3() {
        let card = Card {
            id: 1,
            owned_numbers: vec![69, 82, 63, 72, 16, 21, 14, 1],
            winning_numbers: vec![1, 21, 53, 59, 44],
        };
        assert_eq!(2, card.worth())
    }

    #[test]
    fn card_description_3() {
        let card = Card {
            id: 3,
            owned_numbers: vec![69, 82, 63, 72, 16, 21, 14, 1],
            winning_numbers: vec![1, 21, 53, 59, 44],
        };
        assert_eq!(
            card,
            Card::from("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1")
        )
    }

    #[test]
    fn card_worth_4() {
        let card = Card {
            id: 1,
            owned_numbers: vec![59, 84, 76, 51, 58, 5, 54, 83],
            winning_numbers: vec![41, 92, 73, 84, 69],
        };
        assert_eq!(1, card.worth())
    }

    #[test]
    fn card_description_4() {
        let card = Card {
            id: 4,
            owned_numbers: vec![59, 84, 76, 51, 58, 5, 54, 83],
            winning_numbers: vec![41, 92, 73, 84, 69],
        };
        assert_eq!(
            card,
            Card::from("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83")
        )
    }

    #[test]
    fn card_worth_5() {
        let card = Card {
            id: 1,
            owned_numbers: vec![88, 30, 70, 12, 93, 22, 82, 36],
            winning_numbers: vec![87, 83, 26, 28, 32],
        };
        assert_eq!(0, card.worth())
    }

    #[test]
    fn card_5_how_many_wins() {
        let card = Card {
            id: 1,
            owned_numbers: vec![88, 30, 70, 12, 93, 22, 82, 36],
            winning_numbers: vec![87, 83, 26, 28, 32],
        };
        assert_eq!(0, card.how_many_wins())
    }

    #[test]
    fn card_description_5() {
        let card = Card {
            id: 5,
            owned_numbers: vec![88, 30, 70, 12, 93, 22, 82, 36],
            winning_numbers: vec![87, 83, 26, 28, 32],
        };
        assert_eq!(
            card,
            Card::from("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36")
        )
    }
}

#[cfg(test)]
mod card_pile_tests {
    use super::*;

    #[test]
    fn test_pile_constructor() {
        let card_2 = Card::from("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19");
        let card_5 = Card::from("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36");

        let pile = CardPile::new(vec![card_2, card_5]);

        assert_eq!(2, pile.cards.len());
        assert_eq!(1, pile.card_counts[&2]);
        assert_eq!(1, pile.card_counts[&5]);
    }

    #[test]
    fn test_pile_add_copy() {
        let card_2 = Card::from("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19");
        let card_5 = Card::from("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36");

        let mut pile = CardPile::new(vec![card_2, card_5]);
        pile.add_copy_of_card_with_id(2);
        assert_eq!(2, pile.card_counts[&2]);
    }

    #[test]
    fn test_pile_total_cards() {
        let card_2 = Card::from("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19");
        let card_5 = Card::from("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36");

        let pile = CardPile::new(vec![card_2, card_5]);
        assert_eq!(2, pile.total_cards());
    }
}