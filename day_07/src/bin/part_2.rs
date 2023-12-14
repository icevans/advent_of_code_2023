use day_07::part_2::{CamelCardsHand, CamelCardsHandType};

fn main() {
    let input = include_str!("../input.txt");

    let mut hands: Vec<_> = input
        .lines()
        .map(|line| {
            let hand_description: Vec<&str> = line.split_whitespace().collect();
            let cards = hand_description.first().unwrap();
            let bid: u32 = hand_description.last().unwrap().parse().unwrap();
            CamelCardsHand {
                hand_type: CamelCardsHandType::from(cards),
                bid,
            }
        })
        .collect();

    hands.sort_by(|a, b| a.hand_type.cmp(&b.hand_type));

    let mut total_winnings = 0;
    for i in 0..hands.len() {
        let rank = (i + 1) as u32;
        let worth = hands[i].bid * rank;
        total_winnings += worth;
        println!("{}: {:?}", i + 1, hands[i]);
    }

    println!("{total_winnings}")
}

