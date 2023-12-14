use std::collections::HashMap;

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Clone)]
pub enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl Card {
    pub fn from(c: char) -> Option<Card> {
        match c {
            'J' => Some(Card::Joker),
            '2' => Some(Card::Two),
            '3' => Some(Card::Three),
            '4' => Some(Card::Four),
            '5' => Some(Card::Five),
            '6' => Some(Card::Six),
            '7' => Some(Card::Seven),
            '8' => Some(Card::Eight),
            '9' => Some(Card::Nine),
            'T' => Some(Card::Ten),
            'Q' => Some(Card::Queen),
            'K' => Some(Card::King),
            'A' => Some(Card::Ace),
            _ => None,
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum CamelCardsHandType {
    HighCard([Card; 5]),
    OnePair([Card; 5]),
    TwoPair([Card; 5]),
    ThreeOfAKind([Card; 5]),
    FullHouse([Card; 5]),
    FourOfAKind([Card; 5]),
    FiveOfAKind([Card; 5]),
}

impl CamelCardsHandType {
    pub fn from(hand_string: &str) -> CamelCardsHandType {
        let cards: [Card; 5] = hand_string
            .chars()
            .map(|c| Card::from(c).expect("Invalid card in hand string"))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let mut card_count: HashMap<&Card, u8> = HashMap::new();
        for card in &cards {
            let count = card_count.entry(card).or_insert(0);
            *count += 1;
        }

        let joker_count = card_count.get(&Card::Joker).unwrap_or(&0);

        if *(card_count.get(&cards[0]).unwrap()) == 5 {
            return CamelCardsHandType::FiveOfAKind(cards);
        }

        if card_count.iter().any(|(_, count)| *count == 4) {
            return match joker_count {
                1 | 4 => CamelCardsHandType::FiveOfAKind(cards),
                0 => CamelCardsHandType::FourOfAKind(cards),
                _ => panic!("Impossible number of jokers ({joker_count}) in un-upgraded four of a kind"),
            };
        }

        if card_count.iter().any(|(_, count)| *count == 3)
            && card_count.iter().any(|(_, count)| *count == 2)
        {
            return match joker_count {
                3 | 2 => CamelCardsHandType::FiveOfAKind(cards),
                0 => CamelCardsHandType::FullHouse(cards),
                _ => panic!("Impossible number of jokers ({joker_count}) in un-upgraded full house!"),
            };
        }

        if card_count.iter().any(|(_, count)| *count == 3) {
            return match joker_count {
                1 | 3 => CamelCardsHandType::FourOfAKind(cards),
                0 => CamelCardsHandType::ThreeOfAKind(cards),
                _ => panic!("Impossible number of jokers ({joker_count}) in un-upgraded three of a kind!"),
            };
        }

        if card_count
            .iter()
            .filter(|(_, count)| **count == 2)
            .collect::<Vec<_>>()
            .len()
            == 2
        {
            return match joker_count {
                2 => CamelCardsHandType::FourOfAKind(cards),
                1 => CamelCardsHandType::FullHouse(cards),
                0 => CamelCardsHandType::TwoPair(cards),
                _ => panic!("Impossible number of jokers ({joker_count}) in un-upgraded two pair!"),
            };
        }

        if card_count.iter().any(|(_, count)| *count == 2) {
            return match joker_count {
                2 | 1 => CamelCardsHandType::ThreeOfAKind(cards),
                0 => CamelCardsHandType::OnePair(cards),
                _ => panic!("Impossible number of jokers ({joker_count}) in un-upgraded one pair!"),
            };
        }

        if card_count.iter().all(|(_, count)| *count == 1) {
            return match joker_count {
                1 => CamelCardsHandType::OnePair(cards),
                0 => CamelCardsHandType::HighCard(cards),
                _ => panic!("Impossible number of jokers ({joker_count}) in un-upgraded high card!"),
            };
        }

        panic!("Unexpected hand! {:?}", cards)
    }
}

#[derive(Debug)]
pub struct CamelCardsHand {
    pub hand_type: CamelCardsHandType,
    pub bid: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_five_of_a_kind() {
        let hand_1 = CamelCardsHandType::from("AAAAA");
        let hand_2 = CamelCardsHandType::from("22222");
        assert!(matches!(hand_1, CamelCardsHandType::FiveOfAKind(_)));
        assert!(matches!(hand_2, CamelCardsHandType::FiveOfAKind(_)));
    }

    #[test]
    fn test_four_of_a_kind() {
        let hand_1 = CamelCardsHandType::from("AAAAK");
        let hand_2 = CamelCardsHandType::from("AKKKK");
        assert!(matches!(hand_1, CamelCardsHandType::FourOfAKind(_)));
        assert!(matches!(hand_2, CamelCardsHandType::FourOfAKind(_)));
    }

    #[test]
    fn test_full_house() {
        let hand_1 = CamelCardsHandType::from("AAKKK");
        let hand_2 = CamelCardsHandType::from("AAAKK");
        let hand_3 = CamelCardsHandType::from("JAAKK");
        let hand_4 = CamelCardsHandType::from("JJAAA");
        let hand_5 = CamelCardsHandType::from("JJJKK");
        assert!(matches!(hand_1, CamelCardsHandType::FullHouse(_)));
        assert!(matches!(hand_2, CamelCardsHandType::FullHouse(_)));
        assert!(matches!(hand_3, CamelCardsHandType::FullHouse(_)));
        assert!(matches!(hand_4, CamelCardsHandType::FiveOfAKind(_)));
        assert!(matches!(hand_5, CamelCardsHandType::FiveOfAKind(_)));
    }

    #[test]
    fn test_three_of_a_kind() {
        let hand_1 = CamelCardsHandType::from("T55J5");
        assert!(matches!(hand_1, CamelCardsHandType::FourOfAKind(_)));
    }

    #[test]
    fn test_two_pair() {
        let hand_1 = CamelCardsHandType::from("KK677");
        let hand_2 = CamelCardsHandType::from("KTJJT");
        assert!(matches!(hand_1, CamelCardsHandType::TwoPair(_)));
        assert!(matches!(hand_2, CamelCardsHandType::FourOfAKind(_)));
    }

    #[test]
    fn test_one_pair() {
        let hand = CamelCardsHandType::from("32T3K");
        assert!(matches!(hand, CamelCardsHandType::OnePair(_)))
    }

    #[test]
    fn test_high_card() {
        let hand = CamelCardsHandType::from("32T4K");
        assert!(matches!(hand, CamelCardsHandType::HighCard(_)))
    }

    #[test]
    fn test_full_house_beats_three_of_a_kind() {
        let hand_1 = CamelCardsHandType::from("AAKKK");
        let hand_2 = CamelCardsHandType::from("T55J5");
        assert!(hand_2 > hand_1);
    }

    #[test]
    fn test_two_full_house() {
        let hand_1 = CamelCardsHandType::from("AAKKK");
        let hand_2 = CamelCardsHandType::from("AAAKK");

        assert!(hand_2 > hand_1);
    }

    #[test]
    fn test_two_three_pair() {
        let hand_1 = CamelCardsHandType::from("T55J5");
        let hand_2 = CamelCardsHandType::from("QQQJA");

        assert!(hand_2 > hand_1);
    }

    #[test]
    fn one_pair() {
        let hand = CamelCardsHandType::from("8J382");
        assert!(matches!(hand, CamelCardsHandType::ThreeOfAKind(_)))
    }

    #[test]
    fn one_pair_sample_2() {
        let hand = CamelCardsHandType::from("32T3K");
        assert!(matches!(hand, CamelCardsHandType::OnePair(_)))
    }

    #[test]
    fn two_pair_sample_2() {
        let hand = CamelCardsHandType::from("KK677");
        assert!(matches!(hand, CamelCardsHandType::TwoPair(_)))
    }
}
