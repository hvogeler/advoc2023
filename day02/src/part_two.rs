use log;
use std::{env, fs::read_to_string};

use crate::game::{Game, CubeColor};
pub mod game;

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }
    env_logger::init();

    let lines = read_lines("/Users/hvo/rust/advoc2023/day02/data/data.txt");
    let mut prod_color_counts: u64 = 0;
    log::info!("Advent of code 2023 - Day 02 Part 2");
    for line in lines.iter() {
        let game_result = Game::parse_game(&line);
        match game_result {
            Ok(game) => {
                let red_max = game.get_max_cube_per_color(CubeColor::Red);
                let green_max = game.get_max_cube_per_color(CubeColor::Green);
                let blue_max = game.get_max_cube_per_color(CubeColor::Blue);
                prod_color_counts += u64::from(red_max) * u64::from(green_max) * u64::from(blue_max);
            }
            Err(e) => log::warn!("Result maybe wrong: {}", e),
        }
    }

    log::info!("");
    log::info!("--------------------------------------------------------");
    log::info!("Sum products of color cubes: {}", prod_color_counts);
    log::info!("--------------------------------------------------------");
    log::info!("");
    assert_eq!(prod_color_counts, 70387);
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
