use std::path::Path;

use common::*;

fn main() -> Result<(), Error> {
    // -------------- Part 1 --------------------
    println!("****************************************************************");
    println!("* Day 06  -  Part 1                                            *");
    println!("****************************************************************");
    let test_data = read_test_data(Path::new("./day06/testdata.dat"))?;
    let season = Season::from_str(&test_data).unwrap();
    println!("{:?}", season);
    println!("Complete combinations: {}", season.winning_combinations());

    // -------------- Part 2 --------------------
    println!("****************************************************************");
    println!("* Day 06  -  Part 2                                            *");
    println!("****************************************************************");
    let test_data = read_test_data(Path::new("./day06/testdata.dat"))?;
    let season = Season::from_str2(&test_data).unwrap();
    println!("Complete combinations: {}", season.winning_combinations());

    Ok(())
}

#[derive(Debug, Default)]
pub struct Season {
    races: Vec<Race>,
}

impl Season {
    pub fn add(&mut self, race: Race) {
        self.races.push(race);
    }

    pub fn winning_combinations(&self) -> i64 {
        self.races
            .iter()
            .map(|race| race.calc_win_combinatios().0)
            .reduce(|acc, v| acc * v)
            .unwrap()
    }

    pub fn from_str(data: &str) -> Result<Season, Error> {
        let mut times: Vec<i64> = Vec::new();
        let mut distances: Vec<i64> = Vec::new();

        for line in data.lines() {
            let line = line.to_string() + " ";
            let tokens = lexer(&line)?;

            for i in 1..tokens.len() {
                if let Token::Number(n) = tokens[i] {
                    if tokens[0] == Token::Time {
                        times.push(n);
                    }
                    if tokens[0] == Token::Distance {
                        distances.push(n);
                    }
                }
            }
        }

        let mut season = Season::default();
        for i in 0..times.len() {
            season.add(Race {
                duration: times[i],
                distance: distances[i],
            })
        }
        Ok(season)
    }

    pub fn from_str2(data: &str) -> Result<Season, Error> {
        let mut time: String = String::new();
        let mut distance: String = String::new();

        for line in data.lines() {
            let line = line.to_string() + " ";
            let tokens = lexer(&line)?;

            for i in 1..tokens.len() {
                if let Token::Number(n) = tokens[i] {
                    if tokens[0] == Token::Time {
                        time += &n.to_string();
                    }
                    if tokens[0] == Token::Distance {
                        distance += &n.to_string();
                    }
                }
            }
        }

        let mut season = Season::default();

        season.add(Race {
            duration: time.parse()?,
            distance: distance.parse()?,
        });

        Ok(season)
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Time,
    Distance,
    Number(i64),
}

#[derive(Debug, PartialEq, Default)]
enum ParseState {
    #[default]
    InBlank,
    InReservedWord,
    InNumber,
}

pub fn lexer(line: &str) -> Result<Vec<Token>, Error> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut current_token: Vec<char> = Vec::new();
    let mut parse_state = ParseState::default();
    for char in line.chars() {
        match parse_state {
            ParseState::InBlank => match char {
                'a'..='z' | 'A'..='Z' => {
                    current_token.clear();
                    current_token.push(char);
                    parse_state = ParseState::InReservedWord;
                }
                '0'..='9' => {
                    current_token.clear();
                    current_token.push(char);
                    parse_state = ParseState::InNumber;
                }
                ' ' => {}
                _ => {
                    return Err(Error::SyntaxError(format!(
                        "Unexpected character: {}",
                        char
                    )))
                }
            },
            ParseState::InReservedWord => match char {
                ':' => {
                    let word: String = current_token.iter().collect();
                    if word == "Time" {
                        tokens.push(Token::Time);
                    } else if word == "Distance" {
                        tokens.push(Token::Distance);
                    } else {
                        return Err(Error::SyntaxError(format!("Unknown Word: {}", word)));
                    }
                    current_token.clear();
                    parse_state = ParseState::InBlank;
                }
                'a'..='z' | 'A'..='Z' => {
                    current_token.push(char);
                }
                _ => {
                    return Err(Error::SyntaxError(format!(
                        "Unexpected character '{}' in reserved word: '{}'",
                        char,
                        current_token.iter().collect::<String>()
                    )))
                }
            },
            ParseState::InNumber => match char {
                '0'..='9' => {
                    current_token.push(char);
                }
                ' ' => {
                    let number_str: String = current_token.iter().collect();
                    let number = number_str.parse::<i64>();
                    match number {
                        Ok(n) => tokens.push(Token::Number(n)),
                        Err(e) => return Err(Error::SyntaxError(e.to_string())),
                    }
                    parse_state = ParseState::InBlank;
                }
                _ => {
                    return Err(Error::SyntaxError(format!(
                        "Unexpected character '{}' in number: '{}'",
                        char,
                        current_token.iter().collect::<String>()
                    )))
                }
            },
        }
    }
    Ok(tokens)
}

#[derive(Debug, PartialEq)]
pub struct Race {
    duration: i64,
    distance: i64,
}

impl Race {
    pub fn new(duration: i64, distance: i64) -> Self {
        Self { duration, distance }
    }

    pub fn calc_win_limits(&self) -> WinLimits {
        let race_duration = self.duration;
        let previous_best_distance = self.distance;

        let mut upper_limit = 0i64;
        let mut lower_limit = 0i64;

        // Solve quadratic equation
        let upper_limit_f = (race_duration as f64
            + ((race_duration * race_duration - 4 * previous_best_distance) as f64).sqrt())
            / 2.0;

        // If limit happens to result in equal distance, take the one lower
        if upper_limit_f.floor() == upper_limit_f {
            upper_limit = upper_limit_f.floor() as i64 - 1;
        } else {
            upper_limit = upper_limit_f.floor() as i64;
        }

        // If limit happens to result in equal distance, take the one higher
        let lower_limit_f = (race_duration as f64
            - ((race_duration * race_duration - 4 * previous_best_distance) as f64).sqrt())
            / 2.0;
        if lower_limit_f.floor() == lower_limit_f {
            lower_limit = lower_limit_f.ceil() as i64 + 1;
        } else {
            lower_limit = lower_limit_f.ceil() as i64;
        }
        WinLimits(lower_limit, upper_limit)
    }

    pub fn calc_win_combinatios(&self) -> WinCombinations {
        let limits = self.calc_win_limits();
        WinCombinations(limits.1 - limits.0 + 1)
    }
}

#[derive(Debug, PartialEq)]
pub struct WinLimits(i64, i64);

#[derive(Debug, PartialEq)]
pub struct WinCombinations(i64);

#[cfg(test)]
mod tests {
    use super::*;
    use common::read_test_data;
    use std::path::Path;

    #[test]
    fn test_calc_win_limits() {
        assert_eq!(Race::new(7, 9).calc_win_limits(), WinLimits(2, 5));
        assert_eq!(Race::new(15, 40).calc_win_limits(), WinLimits(4, 11));
        assert_eq!(Race::new(30, 200).calc_win_limits(), WinLimits(11, 19));
        assert_eq!(Race::new(7, 9).calc_win_combinatios(), WinCombinations(4));
        assert_eq!(Race::new(15, 40).calc_win_combinatios(), WinCombinations(8));
        assert_eq!(
            Race::new(30, 200).calc_win_combinatios(),
            WinCombinations(9)
        );
    }

    #[test]
    fn test_season() {
        let test_data = read_test_data(Path::new("./example.dat")).unwrap();
        let season = Season::from_str(&test_data).unwrap();
        println!("{:?}", season);
        assert_eq!(season.races.len(), 3);
        assert_eq!(season.races[1], Race::new(15, 40));
    }

    #[test]
    fn test_season2() {
        let test_data = read_test_data(Path::new("./example.dat")).unwrap();
        let season = Season::from_str2(&test_data).unwrap();
        println!("{:?}", season);
        assert_eq!(season.races.len(), 1);
        assert_eq!(season.races[0], Race::new(71530, 940200));
    }

    #[test]
    fn test_lexer() {
        let test_data = read_test_data(Path::new("./example.dat")).unwrap();
        for (i, line) in test_data.lines().enumerate() {
            let line = line.to_string() + " ";
            if i == 0 {
                assert_eq!(
                    lexer(&line).unwrap(),
                    vec![
                        Token::Time,
                        Token::Number(7),
                        Token::Number(15),
                        Token::Number(30)
                    ]
                );
            }
            if i == 1 {
                assert_eq!(
                    lexer(&line).unwrap(),
                    vec![
                        Token::Distance,
                        Token::Number(9),
                        Token::Number(40),
                        Token::Number(200)
                    ]
                );
            }
        }
    }
}
