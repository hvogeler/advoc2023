use std::path::Path;

use card::Card;
use common::read_test_data;


pub mod card;


fn main() {
    // let example = read_test_data(Path::new("./day04/example.dat")).unwrap();
    let example = read_test_data(Path::new("./day04/testdata.dat")).unwrap();
    let mut sum = 0;
    for row in example.lines() {
        let card = Card::from_card_string(row).unwrap();
        println!("Card {}, score {}", card.card_no, card.score());
        sum += card.score();
    }
    println!("Sum: {}", sum)
}


