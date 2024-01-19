use log;
use std::{
    collections::HashMap, error::Error, fmt, fs::read_to_string, result::Result, str::FromStr, env,
};
use strum_macros::EnumString;

#[derive(Debug)]
struct GameError {
    message: String,
}

impl Error for GameError {}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ERROR(Game Error): {}\n", self.message)
    }
}

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }
    env_logger::init();

    let lines = read_lines("/Users/hvo/rust/advoc2023/day02/data/data.txt");
    let mut sum_ids = 0;
    log::info!("Advent of code 2023 - Day 02");
    for line in lines.iter() {
        let game = Game::parse_game(&line);
        match game {
            Ok(_game) => {
                if _game.is_valid(12, 13, 14) {
                    sum_ids += _game.id;
                }
            }
            Err(e) => log::warn!("Result maybe wrong: {}", e),
        }
    }

    log::info!("");
    log::info!("--------------------------------------------------------");
    log::info!("Sum of IDs of valid games: {}", sum_ids);
    log::info!("--------------------------------------------------------");
    log::info!("");
}

#[derive(Debug)]
struct Game {
    id: u16,
    reveals: Vec<Reveal>,
}

impl Game {
    pub fn parse_game(line: &str) -> Result<Game, GameError> {
        let id_parts = line.split(":").collect::<Vec<&str>>();
        // get the id
        let _id = (&(id_parts[0])[5..]).parse();

        let id = match _id {
            Ok(id) => id,
            Err(_) => {
                return Err(GameError {
                    message: String::from("Game ID could not be parsed"),
                })
            }
        };

        //get reveals
        let game_raw = id_parts[1];
        let reveals_raw = game_raw.split(";").collect::<Vec<&str>>();
        let mut reveals: Vec<Reveal> = vec![];
        for reveal_raw in reveals_raw.iter() {
            let cubes = reveal_raw.trim().split(", ").collect::<Vec<&str>>();
            let mut color_counts: HashMap<CubeColor, u16> = HashMap::new();

            for cube in cubes.iter() {
                let cube_parts = cube.split(" ").collect::<Vec<&str>>();
                let cube_color = CubeColor::from_str(cube_parts[1]).unwrap();
                color_counts.insert(
                    cube_color,
                    cube_parts[0]
                        .parse()
                        .expect("color count could not be parsed"),
                );

                reveals.push(Reveal {
                    red: *color_counts.get(&CubeColor::Red).unwrap_or(&0),
                    green: *(color_counts.get(&CubeColor::Green)).unwrap_or(&0),
                    blue: *color_counts.get(&CubeColor::Blue).unwrap_or(&0),
                });
            }
        }

        let agame = Game {
            id: id,
            reveals: reveals,
        };
        Ok(agame)
    }

    pub fn is_valid(self: &Self, red: u16, green: u16, blue: u16) -> bool {
        for reveal in &self.reveals {
            if reveal.red > red || reveal.green > green || reveal.blue > blue {
                return false;
            }
        }
        true
    }
}

#[derive(Debug, PartialEq, Eq, Hash, EnumString)]
#[strum(ascii_case_insensitive)]
enum CubeColor {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct Reveal {
    red: u16,
    green: u16,
    blue: u16,
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
