use std::{error::Error, fs, path::Path};

const DOT: char = '.';

#[derive(Debug)]
struct Schematic {
    data: Vec<Vec<char>>,
    number_tokens: Vec<NumberToken>,
    symbol_tokens: Vec<SymbolToken>,
}

impl Schematic {
    fn tokenize(&mut self) {
        let mut parse_state = ParseState::BetweenTokens;
        let mut current_token: NumberToken = NumberToken::default();

        for (row_no, row_data) in self.data.iter().enumerate() {
            for (col_no, col_data) in row_data.iter().enumerate() {
                match parse_state {
                    ParseState::BetweenTokens => {
                        if col_data.is_digit(10) {
                            parse_state = ParseState::InNumberToken;
                            current_token = NumberToken::new("".to_string(), row_no, col_no);
                            current_token.n_str.push(*col_data);
                            continue;
                        }
                        if !(col_data.is_digit(10) || col_data == &DOT) {
                            // Symbol token
                            self.symbol_tokens.push(SymbolToken::new(
                                row_no,
                                col_no,
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
                            self.number_tokens
                                .push(current_token);
                            parse_state = ParseState::BetweenTokens;
                            current_token = NumberToken::default();

                            if col_data != &DOT {
                                // Symbol token
                                self.symbol_tokens.push(SymbolToken::new(
                                    row_no,
                                    col_no,
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

impl From<String> for Schematic {
    fn from(v: String) -> Self {
        let mut data: Vec<Vec<char>> = Vec::new();
        for line in v.lines() {
            let mut cv: Vec<char> = line.chars().collect();
            // Wrap lines between '.'
            cv.insert(0, DOT);
            cv.push(DOT);
            data.push(cv);
        }
        Self {
            data,
            number_tokens: Vec::new(),
            symbol_tokens: Vec::new(),
        }
    }
}

enum ParseState {
    BetweenTokens,
    InNumberToken,
}

#[derive(Debug, Default)]
struct NumberToken {
    n_str: String,
    row: usize,
    col: usize,
}

impl NumberToken {
    fn new(n_str: String, row: usize, col: usize) -> Self {
        Self {
            n_str: n_str.clone(),
            row,
            col,
        }
    }

    fn value(&self) -> usize {
        self.n_str.parse().unwrap()
    }

    fn length(&self) -> usize {
        self.n_str.len()
    }
}

#[derive(Debug, PartialEq, Eq)]
enum TokenType {
    Asterisk,
    Any,
}

#[derive(Debug, PartialEq, Eq)]
struct SymbolToken {
    token_type: TokenType,
    row: usize,
    col: usize,
}

impl SymbolToken {
    fn new(row: usize, col: usize, token_type: TokenType) -> Self {
        SymbolToken {
            token_type,
            row,
            col,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let example_data = read_test_data(Path::new("./day03/example.dat"))?;
    let mut schematic: Vec<Vec<char>> = Vec::new();

    println!("{}", example_data);
    Ok(())
}

fn read_test_data(path: &Path) -> Result<String, Box<dyn Error>> {
    let data = fs::read_to_string(path)?;
    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*; // Bring `add` function into scope

    #[test]
    fn test_make_schematic() {
        let example_data = read_test_data(Path::new("./example.dat")).unwrap();
        let sc = Schematic::from(example_data);
        assert_eq!(sc.data.len(), 10);
        assert_eq!(sc.data[0].len(), 12);
        assert_eq!(sc.data[4][3], '7');
    }

    #[test]
    fn test_tokenize() {
        let example_data = read_test_data(Path::new("./example.dat")).unwrap();
        let mut sc = Schematic::from(example_data);
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
