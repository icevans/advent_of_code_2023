use day_04::{Card, CardPile};

fn main() {
    let input = include_str!("../input.txt");
    let cards: Vec<Card> = input
        .lines()
        .map(|line| Card::from(line))
        .collect();

    let mut card_pile = CardPile::new(cards);
    let greatest_card_id_in_pile = card_pile.cards.last().unwrap().id;

    for i in 1..=greatest_card_id_in_pile {
        let card = card_pile.get_original_card_by_id(i);
        let wins = card.how_many_wins();

        // Do this part once for every copy of the current card
        for _ in 0..card_pile.card_counts[&card.id] {
            for j in i+1..=i + wins {
                card_pile.add_copy_of_card_with_id(j);
            }
        }
    }

    println!("{}", card_pile.total_cards());
}
