use crate::card::Card;

#[derive(Debug, Default)]
pub struct Deck {
    cards: Vec<CardWithCount>,
}

impl Deck {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, card: Card) {
        self.cards.push(CardWithCount { card, count: 1});
    }

    pub fn process_wins(&mut self) {
        for i in 0..self.cards.len() {
            let card_with_count = &self.cards[i];
            let count = card_with_count.count;
            let winning_numbers = card_with_count.card().correct_numbers();
            
            for j in (i+1)..(i+1+winning_numbers.len()) {
                if j >= self.cards.len() { break; }
                self.cards[j].add_copies(count); 
            }
        }
    }

    pub fn number_of_cards(&self) -> i64 {
        self.cards.iter().map(|card_with_count| card_with_count.count).reduce(|acc, count| acc + count).unwrap()
    }
}

/// A card with a count of the number of copies of it in the deck.
#[derive(Debug, Default)]
pub struct CardWithCount {
    card: Card,
    count: i64,
}

impl CardWithCount {
    pub fn card(&self) -> &Card {
        &self.card
    }

    pub fn add_copies(&mut self, count: i64) {
        self.count += count;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck() {
        let mut deck = Deck::new();
        for line in TEST_CARDS.lines() {
            deck.add(Card::from_card_string(line).unwrap());
        }

        assert_eq!(deck.cards.len(), 6);
        assert_eq!(deck.cards[0].count, 1);
    }

    #[test]
    fn test_process_wins() {
        let mut deck = Deck::new();
        for line in TEST_CARDS.lines() {
            deck.add(Card::from_card_string(line).unwrap());
        }
        deck.process_wins();
        assert_eq!(deck.cards.len(), 6);
        assert_eq!(deck.cards[0].count, 1);
        assert_eq!(deck.cards[1].count, 2);
        assert_eq!(deck.cards[2].count, 4);
        assert_eq!(deck.cards[3].count, 8);
        assert_eq!(deck.cards[4].count, 14);
        assert_eq!(deck.cards[5].count, 1);

        assert_eq!(deck.number_of_cards(), 30);
    }

    const TEST_CARDS: &'static str = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53 
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
}
