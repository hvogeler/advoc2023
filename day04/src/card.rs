use std::usize;

#[derive(Debug, Default)]
pub struct Card {
    pub card_no: usize,
    pub wins: Vec<usize>,
    pub played: Vec<usize>,
}

impl Card {
    pub fn from_card_string(card_str: &str) -> Result<Self, ParseError> {
        let tokens = lexer(card_str)?;
        let mut card = Self {
            card_no: if let Token::Number(n) = tokens[1] {
                n
            } else {
                0
            },
            ..Self::default()
        };
        let mut is_winner = true;
        for i in 3..tokens.len() {
            if tokens[i] == Token::Pipe {
                is_winner = false;
            }
            if let Token::Number(n) = tokens[i] {
                if is_winner {
                    card.wins.push(n);
                } else {
                    card.played.push(n);
                }
            }
        }
        Ok(card)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Card,
    Colon,
    Pipe,
    Number(usize),
    WinningNumber(usize),
    PlayedNumber(usize),
}

#[derive(Debug, Default)]
pub enum ParseState {
    InToken,
    #[default]
    OutsideToken,
}

#[derive(Debug)]
pub enum ParseError {
    SyntaxError,
}

fn lexer(card: &str) -> Result<Vec<Token>, ParseError> {
    let mut tokens = Vec::new();
    let mut tmp_token: Vec<char> = Vec::new();
    let mut parse_state = ParseState::default();
    let mut cursor: usize = 0;
    let mut chars = Vec::from_iter(card.chars());
    chars.push(' ');
    let mut current_char = chars.get(0);
    while let Some(_char) = current_char {
        match parse_state {
            ParseState::OutsideToken => match _char {
                '|' => {
                    tokens.push(Token::Pipe);
                    current_char = advance(&chars, &mut cursor);
                }
                '0'..='9' => {
                    parse_state = ParseState::InToken;
                    tmp_token.clear();
                    tmp_token.push(*_char);
                    current_char = advance(&chars, &mut cursor);
                }
                'A'..='Z' => {
                    parse_state = ParseState::InToken;
                    tmp_token.clear();
                    tmp_token.push(*_char);
                    current_char = advance(&chars, &mut cursor);
                }
                ' ' => current_char = advance(&chars, &mut cursor),
                _ => return Err(ParseError::SyntaxError),
            },
            ParseState::InToken => match _char {
                ':' => {
                    let card_no: usize = String::from_iter(tmp_token.iter()).parse().unwrap();
                    tokens.push(Token::Number(card_no));
                    tokens.push(Token::Colon);
                    tmp_token.clear();
                    parse_state = ParseState::OutsideToken;
                    current_char = advance(&chars, &mut cursor);
                }
                '0'..='9' => {
                    tmp_token.push(*_char);
                    current_char = advance(&chars, &mut cursor);
                }
                'a'..='z' => {
                    tmp_token.push(*_char);
                    current_char = advance(&chars, &mut cursor);
                }
                ' ' => {
                    let s: String = String::from_iter(tmp_token.iter());
                    if s == "Card" {
                        tokens.push(Token::Card);
                    } else if let Ok(n) = s.parse::<usize>() {
                        tokens.push(Token::Number(n));
                    } else {
                        println!("SyntaxError: {}", String::from_iter(tmp_token.iter()));
                    }

                    tmp_token.clear();
                    parse_state = ParseState::OutsideToken;
                    current_char = advance(&chars, &mut cursor);
                }
                _ => return Err(ParseError::SyntaxError),
            },
        }
    }
    Ok(tokens)
}

fn advance<'a>(chars: &'a Vec<char>, cursor: &mut usize) -> Option<&'a char> {
    *cursor += 1;
    chars.get(*cursor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_card() {
        let tokens = lexer(test_card_1).unwrap();
        println!("TOKENS: {:?}", tokens);
        assert_eq!(tokens.len(), 17);
    }

    #[test]
    fn test_card() {
        let card = Card::from_card_string(test_card_1).unwrap();
        println!("CARD: {:?}", card);
        assert_eq!(card.wins.len(), 5);
    }
    const test_card_1: &'static str = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
}
