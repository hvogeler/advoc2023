use log;
use std::{env, fs::read_to_string};
use crate::game::Game;

pub mod game;

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
    assert_eq!(sum_ids, 1734);
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
