use std::fs::read_to_string;
// use std::io;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    let lines = read_lines("/Users/hvo/rust/advoc2023/day02/data/data.txt");
    let mut sum_ids = 0;

    for line in lines.iter() {
        let game = parse_game(&line).expect("game could not be parsed");
        if game.is_valid(12, 13, 14) {
            sum_ids += game.id;
        }
    }

    println!("Sum of IDs of invalid games: {}", sum_ids);
}

fn parse_game(line: &str) -> Result<Game> {
    let id_parts = line.split(":").collect::<Vec<&str>>();
    // get the id
    let id: u16 = (&(id_parts[0])[5..])
        .parse()
        .expect("id could not be parsed");

    //get reveals
    let game_raw = id_parts[1];
    let reveals_raw = game_raw.split(";").collect::<Vec<&str>>();
    let mut reveals: Vec<Reveal> = vec![];
    for reveal_raw in reveals_raw.iter() {
        let cubes = reveal_raw.trim().split(", ").collect::<Vec<&str>>();
        let mut red: u16 = 0;
        let mut green: u16 = 0;
        let mut blue: u16 = 0;

        for cube in cubes.iter() {
            let cube_parts = cube.split(" ").collect::<Vec<&str>>();
            if cube_parts[1] == "red" {
                red = cube_parts[0]
                    .parse()
                    .expect("color count could not be parsed");
            } else if cube_parts[1] == "green" {
                green = cube_parts[0]
                    .parse()
                    .expect("color count could not be parsed");
            } else if cube_parts[1] == "blue" {
                blue = cube_parts[0]
                    .parse()
                    .expect("color count could not be parsed");
            }
        }
        reveals.push(Reveal { red, green, blue });
    }

    let agame = Game {
        id: id,
        reveals: reveals,
    };
    Ok(agame)
}

#[derive(Debug)]
struct Game {
    id: u16,
    reveals: Vec<Reveal>,
}

impl Game {
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
