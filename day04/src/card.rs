use std::{fmt, usize};
use std::{default, error::Error};

#[derive(Debug)]
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


fn parse(card: &str) -> Result<Vec<Token>, ParseError> {
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
                },
                '0'..='9' => {
                    parse_state = ParseState::InToken;
                    tmp_token.clear();
                    tmp_token.push(*_char);
                    current_char = advance(&chars, &mut cursor);
                },
                'A'..='Z' => {
                    parse_state = ParseState::InToken;
                    tmp_token.clear();
                    tmp_token.push(*_char);
                    current_char = advance(&chars, &mut cursor);
                },
                ' ' => current_char = advance(&chars, &mut cursor),
                _ => return Err(ParseError::SyntaxError),

            },
            ParseState::InToken => {
                match _char {
                    ':' => {
                        let card_no: usize = String::from_iter(tmp_token.iter()).parse().unwrap();
                        tokens.push(Token::Number(card_no));
                        tokens.push(Token::Colon);
                        tmp_token.clear();
                        parse_state = ParseState::OutsideToken;
                        current_char = advance(&chars, &mut cursor);
                    },
                    '0'..='9' => {
                        tmp_token.push(*_char);
                        current_char = advance(&chars, &mut cursor);
                    },
                    'a'..='z' => {
                        tmp_token.push(*_char);
                        current_char = advance(&chars, &mut cursor);
                    },
                    ' ' => {
                        let s: String = String::from_iter(tmp_token.iter());
                        if s == "Card" {
                            tokens.push(Token::Card);
                        } else 
                        if let Ok(n) = s.parse::<usize>() {
                            tokens.push(Token::Number(n));
                        } else  {
                            println!("SyntaxError: {}", String::from_iter(tmp_token.iter()));
                        }
                        
                        tmp_token.clear();
                        parse_state = ParseState::OutsideToken;
                        current_char = advance(&chars, &mut cursor);
                    },
                    _ => return Err(ParseError::SyntaxError),
                }
            }
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
        let tokens = parse(&test_card_1).unwrap();
        println!("TOKENS: {:?}", tokens);
        assert_eq!(tokens.len(), 17);

    }

    const test_card_1: &'static str = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
}
