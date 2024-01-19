use std::{collections::HashMap, error::Error, fmt, result::Result, str::FromStr};
use strum_macros::EnumString;

#[derive(Debug)]
pub struct Game {
    pub id: u16,
    reveals: Vec<Reveal>,
}

impl Game {
    // A game line looks like this:
    // Game 2: 3 red, 1 blue, 2 green; 1 blue, 9 green; 1 red, 10 green
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

        Ok(Game { id, reveals })
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

#[derive(Debug)]
pub struct GameError {
    pub message: String,
}

impl Error for GameError {}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ERROR(Game Error): {}\n", self.message)
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
