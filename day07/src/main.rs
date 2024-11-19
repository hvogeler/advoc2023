use std::{collections::HashMap, path::Path, str::FromStr};

use common::{read_test_data, Error};
use strum::VariantArray;
use strum_macros::{EnumString, VariantArray};
use tracing::info;

fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_thread_ids(true)
        .with_target(false)
        .with_level(true)
        .with_thread_names(true)
        .with_line_number(true)
        .init();

    let pkg_name = env!("CARGO_PKG_NAME");
    // -------------- Part 1 --------------------
    info!("****************************************************************");
    info!("* {}  -  Part 1                                             *", pkg_name);
    info!("****************************************************************");
    let test_data = read_test_data(Path::new("./example.dat"))?;

    // -------------- Part 2 --------------------
    info!("****************************************************************");
    info!("* {}  -  Part 2                                             *", pkg_name);
    info!("****************************************************************");

    Ok(())
}

#[derive(Debug, PartialEq, Eq, Hash, EnumString, VariantArray, Default, Clone)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
    #[default]
    Nothing,
}

impl HandType {
    pub fn prio(&self) -> i64 {
        for (i, variant) in Self::VARIANTS.iter().enumerate() {
            if variant == self {
                return (Self::VARIANTS.len() - i) as i64;
            }
        }
        0
    }
}

#[derive(Debug, Default)]
struct Hands {
    hands: Vec<Hand>,
}

impl Hands {
    pub fn from_str(s: &str) -> Result<Self, Error> {
        let mut hands = Hands::default();
        for line in s.lines() {
            hands.hands.push(Hand::from_str(line)?);
        }
        Ok(hands)
    }
}

#[derive(Debug, Default, PartialEq)]
struct Hand {
    cards: [Card; 5],
    bid: i64,
}

impl Hand {
    pub fn from_str(line: &str) -> Result<Self, Error> {
        let mut hand = Self::default();
        let mut parts = line.split(" ");
        let cards_str = parts.next().unwrap();
        for (i, card_char) in cards_str.chars().enumerate() {
            let card_face = CardFace::from_str(card_char.to_string().as_str())?;
            hand.cards[i].prio = card_face.prio();
            hand.cards[i].face = card_face;
        }
        hand.bid = parts.next().unwrap().parse().unwrap();
        Ok(hand)
    }

    pub fn score(&self) -> HandType {
        let mut card_counts: HashMap<Card, i64> = HashMap::new();
        for i in 0..self.cards.len() {
            if card_counts.contains_key(&self.cards[i]) {
                let count = card_counts.get_mut(&self.cards[i]).unwrap();
                *count += 1;
            } else {
                card_counts.insert(self.cards[i].clone(), 1);
            }
        }
        let mut res: Vec<i64> = card_counts.values().cloned().collect();
        res.sort();
        res.reverse();

        if res.len() == 1 {
            return HandType::FiveOfAKind;
        }
        if res.len() == 5 {
            return HandType::HighCard;
        }
        if res.len() == 2 {
            if res[0] == 3 {
                return HandType::FullHouse;
            }
            if res[0] == 4 {
                return HandType::FourOfAKind;
            }
        }
        if res.len() == 3 {
            if res[0] == 3 {
                return HandType::ThreeOfAKind;
            }
            if res[0] == 2 {
                return HandType::TwoPair;
            }
        }
        if res.len() == 4 {
            return HandType::OnePair;
        }
        HandType::Nothing
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let score_self = self.score();
        let score_other = other.score();

        if self.cards[0].prio < other.cards[0].prio {
            return Some(std::cmp::Ordering::Less);
        }
        if self.cards[0].prio > other.cards[0].prio {
            return Some(std::cmp::Ordering::Greater);
        }
        Some(std::cmp::Ordering::Equal)
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct Card {
    face: CardFace,
    prio: i64,
}

#[derive(Debug, PartialEq, Eq, Hash, EnumString, VariantArray, Default, Clone)]
enum CardFace {
    #[default]
    #[strum(serialize = "A")]
    Ace,

    #[strum(serialize = "K")]
    King,

    #[strum(serialize = "Q")]
    Queen,

    #[strum(serialize = "J")]
    Jack,

    #[strum(serialize = "T")]
    Ten,

    #[strum(serialize = "9")]
    Nine,

    #[strum(serialize = "8")]
    Eight,

    #[strum(serialize = "7")]
    Seven,

    #[strum(serialize = "6")]
    Six,

    #[strum(serialize = "5")]
    Five,

    #[strum(serialize = "4")]
    Four,

    #[strum(serialize = "3")]
    Three,

    #[strum(serialize = "2")]
    Two,
}

impl CardFace {
    pub fn prio(&self) -> i64 {
        for (i, variant) in Self::VARIANTS.iter().enumerate() {
            if variant == self {
                return (Self::VARIANTS.len() - i) as i64;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::read_test_data;
    use std::path::Path;

    #[test]
    fn test_hand_type() {
        assert_eq!(HandType::FiveOfAKind.prio(), 8);
        assert_eq!(HandType::ThreeOfAKind.prio(), 5);
        assert_eq!(HandType::Nothing.prio(), 1);
    }

    #[test]
    fn test_score() {
        let test_data = read_test_data(Path::new("./example.dat")).unwrap();
        let hands = Hands::from_str(&test_data).unwrap();
        for hand in hands.hands.iter() {
            println!("{:?}", hand.score());
        }
        let scores: Vec<HandType> = hands.hands.iter().map(|hand| hand.score()).collect();
        assert_eq!(
            scores,
            vec![
                HandType::OnePair,
                HandType::ThreeOfAKind,
                HandType::TwoPair,
                HandType::TwoPair,
                HandType::ThreeOfAKind
            ]
        );

        let more_hands = Hands::from_str(
            r#"TTTTT 1
34567 8
T2TTT 2
32323 2
"#,
        )
        .unwrap();
        let scores: Vec<HandType> = more_hands.hands.iter().map(|hand| hand.score()).collect();
        assert_eq!(scores[0], HandType::FiveOfAKind);
        assert_eq!(scores[1], HandType::HighCard);
        assert_eq!(scores[2], HandType::FourOfAKind);
        assert_eq!(scores[3], HandType::FullHouse);
    }

    #[test]
    fn test_hands_2() {
        let test_data = read_test_data(Path::new("./example.dat")).unwrap();
        let hands = Hands::from_str(&test_data).unwrap();
        assert!(hands.hands[0] < hands.hands[1]);
        assert!(hands.hands[0] <= hands.hands[0]);
        assert!(hands.hands[0] == hands.hands[0]);
        assert!(hands.hands[2] > hands.hands[1]);
    }

    #[test]
    fn test_hands_1() {
        let test_data = read_test_data(Path::new("./example.dat")).unwrap();
        let hands = Hands::from_str(&test_data).unwrap();
        for (i, hand) in hands.hands.iter().enumerate() {
            println!("{:?}", hand);
            if i == 0 {
                assert_eq!(
                    hand.cards,
                    [
                        Card {
                            face: CardFace::Three,
                            prio: 2
                        },
                        Card {
                            face: CardFace::Two,
                            prio: 1
                        },
                        Card {
                            face: CardFace::Ten,
                            prio: 9
                        },
                        Card {
                            face: CardFace::Three,
                            prio: 2
                        },
                        Card {
                            face: CardFace::King,
                            prio: 12
                        }
                    ]
                );
                assert_eq!(hand.bid, 765);
            }
            if i == 4 {
                assert_eq!(
                    hand.cards,
                    [
                        Card {
                            face: CardFace::Queen,
                            prio: 11
                        },
                        Card {
                            face: CardFace::Queen,
                            prio: 11
                        },
                        Card {
                            face: CardFace::Queen,
                            prio: 11
                        },
                        Card {
                            face: CardFace::Jack,
                            prio: 10
                        },
                        Card {
                            face: CardFace::Ace,
                            prio: 13
                        }
                    ]
                );
                assert_eq!(hand.bid, 483);
            }
        }
    }

    #[test]
    fn test_prio_map() {
        // let prio_map = CardPrio::new();
        assert_eq!(CardFace::Ace.prio(), 13);
        assert!(CardFace::Ace.prio() > CardFace::King.prio());
        assert_eq!(CardFace::Ten.prio(), 9);
        assert_eq!(CardFace::Two.prio(), 1);
    }
}
