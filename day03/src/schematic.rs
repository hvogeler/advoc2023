
const BLANK: char = '.';

#[derive(Debug, Default)]
pub struct Schematic {
    data: Vec<Vec<char>>,
    number_tokens: Vec<NumberToken>,
    symbol_tokens: Vec<SymbolToken>,
    pub part_numbers: Vec<PartNumber>,
    pub gears: Vec<Gear>,
}

impl Schematic {
    pub fn from_string(s: &str) -> Self {
        let mut schematic = Schematic::from(s);
        schematic.tokenize();
        schematic.find_part_numbers();
        schematic.find_gears();
        schematic
    }

    fn find_part_numbers(&mut self) {
        for number_token in self.number_tokens.iter() {
            let mut number_token = number_token.clone();
            if let Some(symbol_token) = self.is_next_to_symbol(&number_token) {
                number_token.symbol_token = Some(symbol_token);
                self.part_numbers.push(PartNumber(number_token));
            }
        }
    }

    fn find_gears(&mut self) {
        let possible_gears: Vec<&PartNumber> = self
            .part_numbers
            .iter()
            .filter(|pn| {
                pn.0.symbol_token
                    .clone()
                    .is_some_and(|symbol| symbol.token_type == TokenType::Asterisk)
            })
            .collect();
        for (i, pn1) in possible_gears.iter().enumerate() {
            for j in (i + 1)..possible_gears.len() {
                if pn1.0.symbol_token() == possible_gears[j].0.symbol_token() {
                    self.gears
                        .push(Gear::new(pn1.0.clone(), possible_gears[j].0.clone()));
                }
            }
        }
    }

    fn is_next_to_symbol(&self, number_token: &NumberToken) -> Option<SymbolToken> {
        for symbol_token in self.symbol_tokens.iter() {
            let symbol_token: SymbolToken = symbol_token.clone();
            if symbol_token.row >= number_token.row - 1 && symbol_token.row <= number_token.row + 1
            {
                if symbol_token.col >= number_token.col - 1
                    && symbol_token.col <= number_token.col + number_token.length() as i64
                {
                    return Some(symbol_token.clone());
                }
            }
        }
        None
    }

    fn tokenize(&mut self) {
        let mut parse_state = ParseState::BetweenTokens;
        let mut current_token: NumberToken = NumberToken::default();

        for (row_no, row_data) in self.data.iter().enumerate() {
            for (col_no, col_data) in row_data.iter().enumerate() {
                match parse_state {
                    ParseState::BetweenTokens => {
                        if col_data.is_digit(10) {
                            parse_state = ParseState::InNumberToken;
                            current_token =
                                NumberToken::new("".to_string(), row_no as i64, col_no as i64);
                            current_token.n_str.push(*col_data);
                            continue;
                        }
                        if !(col_data.is_digit(10) || col_data == &BLANK) {
                            // Symbol token
                            self.symbol_tokens.push(SymbolToken::new(
                                row_no as i64,
                                col_no as i64,
                                if *col_data == '*' {
                                    TokenType::Asterisk
                                } else {
                                    TokenType::Any
                                },
                            ));
                        }
                    }
                    ParseState::InNumberToken => {
                        if !col_data.is_digit(10) {
                            self.number_tokens.push(current_token);
                            parse_state = ParseState::BetweenTokens;
                            current_token = NumberToken::default();

                            if col_data != &BLANK {
                                // Symbol token
                                self.symbol_tokens.push(SymbolToken::new(
                                    row_no as i64,
                                    col_no as i64,
                                    if *col_data == '*' {
                                        TokenType::Asterisk
                                    } else {
                                        TokenType::Any
                                    },
                                ));
                            }
                            continue;
                        }

                        current_token.n_str.push(*col_data);
                    }
                }
            }
        }
    }
}

impl From<&str> for Schematic {
    fn from(v: &str) -> Self {
        let mut data: Vec<Vec<char>> = Vec::new();
        for line in v.lines() {
            let mut cv: Vec<char> = line.chars().collect();
            // Wrap lines between '.'
            cv.insert(0, BLANK);
            cv.push(BLANK);
            data.push(cv);
        }
        Self {
            data,
            ..Default::default()
        }
    }
}

#[derive(Debug)]
pub struct PartNumber(pub NumberToken);

#[derive(Debug, Clone)]
pub struct Gear {
    part_numbers: (NumberToken, NumberToken),
}

impl Gear {
    pub fn new(part_number_1: NumberToken, part_number_2: NumberToken) -> Self {
        Self {
            part_numbers: (part_number_1, part_number_2),
        }
    }

    pub fn ratio(&self) -> usize {
        self.part_numbers.0.value() * self.part_numbers.1.value()
    }
}

enum ParseState {
    BetweenTokens,
    InNumberToken,
}

#[derive(Debug, Default, Clone)]
pub struct NumberToken {
    n_str: String,
    row: i64,
    col: i64,
    symbol_token: Option<SymbolToken>,
}

impl NumberToken {
    fn new(n_str: String, row: i64, col: i64) -> Self {
        Self {
            n_str: n_str.clone(),
            row,
            col,
            ..Default::default()
        }
    }

    pub fn symbol_token(&self) -> Option<SymbolToken> {
        self.symbol_token.clone()
    }

    pub fn value(&self) -> usize {
        self.n_str.parse().unwrap()
    }

    fn length(&self) -> usize {
        self.n_str.len()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    Asterisk,
    Any,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SymbolToken {
    pub token_type: TokenType,
    pub row: i64,
    pub col: i64,
}

impl SymbolToken {
    pub fn new(row: i64, col: i64, token_type: TokenType) -> Self {
        SymbolToken {
            token_type,
            row,
            col,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::read_test_data;
    use std::path::Path; // Bring `add` function into scope

    #[test]
    fn test_make_schematic() {
        let example_data = read_test_data(Path::new("./example.dat")).unwrap();
        let sc = Schematic::from(example_data.as_str());
        assert_eq!(sc.data.len(), 10);
        assert_eq!(sc.data[0].len(), 12);
        assert_eq!(sc.data[4][3], '7');
    }

    #[test]
    fn test_part_numbers() {
        let example_data = read_test_data(Path::new("./example.dat")).unwrap();
        let mut sc = Schematic::from(example_data.as_str());
        sc.tokenize();
        sc.find_part_numbers();
        println!("Part Numbers: {:?}", sc.part_numbers);
        let vv = sc
            .part_numbers
            .iter()
            .map(|pn| pn.0.value())
            .reduce(|acc, v| acc + v)
            .unwrap();
        println!("Sum of part numbers: {}", vv);
        assert_eq!(vv, 4361);
    }

    #[test]
    fn test_gears() {
        let example_data = read_test_data(Path::new("./example.dat")).unwrap();
        let mut sc = Schematic::from(example_data.as_str());
        sc.tokenize();
        sc.find_part_numbers();
        sc.find_gears();
        assert_eq!(sc.gears.len(), 2);
        let sum_gear_ratios = sc
            .gears
            .iter()
            .map(|gear| gear.ratio())
            .reduce(|acc, v| acc + v)
            .unwrap();
        assert_eq!(sum_gear_ratios, 467835)
    }

    #[test]
    fn test_tokenize() {
        let example_data = read_test_data(Path::new("./example.dat")).unwrap();
        let mut sc = Schematic::from(example_data.as_str());
        sc.tokenize();
        assert_eq!(sc.number_tokens.len(), 10);
        assert_eq!(sc.symbol_tokens.len(), 6);
        assert_eq!(sc.number_tokens[4].value(), 617);
        assert_eq!(sc.number_tokens[4].col, 1);
        assert_eq!(sc.number_tokens[4].row, 4);
        assert_eq!(sc.number_tokens[4].length(), 3);
        assert_eq!(sc.symbol_tokens[2].token_type, TokenType::Asterisk);
        assert_eq!(sc.symbol_tokens[2].col, 4);
        assert_eq!(sc.symbol_tokens[2].row, 4);
    }

    #[test]
    fn test_symbol_token() {
        assert_eq!(
            SymbolToken::new(1, 3, TokenType::Any),
            SymbolToken::new(1, 3, TokenType::Any)
        );
        assert_ne!(
            SymbolToken::new(1, 3, TokenType::Any),
            SymbolToken::new(1, 3, TokenType::Asterisk)
        );
        assert_ne!(
            SymbolToken::new(1, 2, TokenType::Any),
            SymbolToken::new(1, 3, TokenType::Any)
        );
    }
}
