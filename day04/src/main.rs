use card::Card;
use common::read_test_data;
use deck::Deck;
use std::path::Path;

mod card;
mod deck;

fn main() {
    // let example = read_test_data(Path::new("./day04/example.dat")).unwrap();
    let data = read_test_data(Path::new("./day04/testdata.dat")).unwrap();

    // ------------ Part 1 ----------------
    let mut sum = 0;
    for row in data.lines() {
        let card = Card::from_card_string(row).unwrap();
        // println!("Card {}, score {}", card.card_no, card.score());
        sum += card.score();
    }
    println!("Sum: {}", sum);

    let sum1 = data
        .lines()
        .into_iter()
        .map(|row| {
            let card = Card::from_card_string(row).unwrap();
            // println!("Card: {}, score {}", card.card_no, card.score());
            card.score()
        })
        .reduce(|acc, score| acc + score)
        .unwrap();
    println!("Sum1: {}", sum1);

    // ------------ Part 2 ----------------
    let mut deck = Deck::new();
    for line in data.lines() {
        deck.add(Card::from_card_string(line).unwrap());
    }

    deck.process_wins();
    println!("Number of cards in deck: {}", deck.number_of_cards());
}
