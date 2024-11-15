use std::{error::Error, i64, path::Path, time::Instant};

use common::read_test_data;
use num_format::{Locale, ToFormattedString};
use tokio::task::JoinSet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // -------------- Part 1 --------------------
    println!("****************************************************************");
    println!("* Day 05  -  Part 1                                            *");
    println!("****************************************************************");
    let test_data = read_test_data(&Path::new("./testdata.dat")).unwrap();
    let map_chain = MapChain::from_str(&test_data);
    let seeds = get_seeds(&test_data);
    let locations: Vec<i64> = seeds.into_iter().map(|seed| map_chain.map(seed)).collect();
    println!("Locations: {:?}", locations.iter().map(|n| n.to_formatted_string(&Locale::de)).map(|s| s + " | ").collect::<String>());
    let min = locations
        .iter()
        .reduce(|acc, v| if v < acc { v } else { acc })
        .unwrap();
    println!("  Minimum location: {}", min.to_formatted_string(&Locale::de));

    // -------------- Part 2 --------------------
    println!("");
    println!("****************************************************************");
    println!("* Day 05  -  Part 2                                            *");
    println!("****************************************************************");
    let seeds = get_seeds(&test_data);
    let mut seed_count = 0;
    for i in (0..seeds.len()).step_by(2) {
        seed_count += seeds[i + 1];
    }
    println!("Number of seeds: {:>14}", seed_count.to_formatted_string(&Locale::de));
    let mut join_set = JoinSet::new();

    for i in (0..seeds.len()).step_by(2) {
        {
            let start_seed = seeds[i];
            let end_seed = start_seed + seeds[i + 1];
            let map_chain = map_chain.clone();
            join_set.spawn(async move {
                println!("{:>3}. Spawning seeds: {:>14} - {:>14}, length: {:>14}", 
                (i+2)/2, 
                start_seed.to_formatted_string(&Locale::de), 
                end_seed.to_formatted_string(&Locale::de), 
                (end_seed - start_seed).to_formatted_string(&Locale::de));
                let mut min_loc = i64::MAX;
                for j in start_seed..end_seed {
                    let loc = &map_chain.map(j);
                    if loc < &min_loc {
                        min_loc = *loc;
                    }
                }
                min_loc
            });
        }
    }

    // let results = join_set.join_all().await;
    let start = Instant::now();
    let mut cnt = 0;
    let mut min_loc = i64::MAX;
    while let Some(result) = join_set.join_next().await {
        let min_of_seed = result.unwrap();
        cnt += 1;
        println!("{:>3}. Process finished. min_loc: {:>14}", cnt, min_of_seed.to_formatted_string(&Locale::de));
        if min_of_seed < min_loc {
            min_loc = min_of_seed;
        }
    }

    println!("Result of Day05 part2: {}", min_loc.to_formatted_string(&Locale::de));
    println!("Process took {} seconds", start.elapsed().as_secs());

    Ok(())
}

pub fn get_seeds(data: &str) -> Vec<i64> {
    let mut seeds: Vec<i64> = Vec::new();
    for line in data.lines() {
        if line.starts_with(SEED_MARKER) {
            let seeds_strs = line.split(SEED_MARKER).skip(1).collect::<String>();
            for seed_str in seeds_strs.split(" ") {
                seeds.push(seed_str.parse().unwrap());
            }
            break;
        }
    }
    seeds
}

const SEED_MARKER: &'static str = "seeds: ";
const MAP_MARKER: &'static str = " map:";

#[derive(Debug, Default, Clone)]
pub struct MapChain {
    maps: Vec<NamedMap>,
}

impl MapChain {
    pub fn map(&self, v: i64) -> i64 {
        // println!("Start: {}", v);
        let mut map_staged = v;
        for map in &self.maps {
            map_staged = map.map(map_staged);
            // println!("  Map {}: -> {}", map.name(), map_staged);
        }
        map_staged
    }

    pub fn from_str(data: &str) -> Self {
        let mut map_chain = MapChain::default();
        let mut parse_state = ParseState::BlankLine;
        let mut current_raw_map = NamedMap::default();

        // add a blank line to properly let the parser finish the last block
        for line in data.lines().chain((vec!["", ""]).into_iter()) {
            match parse_state {
                ParseState::BlankLine => {
                    if line.starts_with(SEED_MARKER) {
                        parse_state = ParseState::BlankLine;
                    }
                    if line.contains(MAP_MARKER) {
                        current_raw_map = NamedMap::default();
                        current_raw_map.name = line.split(" ").next().unwrap().to_owned();
                        parse_state = ParseState::MapBlock;
                    }
                }
                ParseState::MapBlock => {
                    if line.is_empty() {
                        map_chain.maps.push(current_raw_map.clone());
                        current_raw_map = NamedMap::default();
                        parse_state = ParseState::BlankLine;
                    } else {
                        current_raw_map.map_entries.push(MapEntry::from(line));
                    }
                }
                ParseState::SeedLine => {}
            }
        }
        map_chain
    }
}

#[derive(Debug, Default, Clone)]
pub struct NamedMap {
    name: String,
    map_entries: Vec<MapEntry>,
}

impl NamedMap {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn map_entries(&self) -> &[MapEntry] {
        self.map_entries.as_slice()
    }

    pub fn map(&self, v: i64) -> i64 {
        for map_entry in &self.map_entries {
            let mapped_value = map_entry.map(v);
            if mapped_value != v {
                return mapped_value;
            }
        }
        v
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseState {
    BlankLine,
    SeedLine,
    MapBlock,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MapEntry {
    target_start: i64,
    source_start: i64,
    range_length: i64,
}

impl MapEntry {
    pub fn map(&self, v: i64) -> i64 {
        if v >= self.source_start && v < self.source_start + self.range_length {
            v + self.target_start - self.source_start
        } else {
            v
        }
    }
}

impl From<&str> for MapEntry {
    fn from(v: &str) -> Self {
        let mut parts = [0i64; 3];
        for (i, n) in v.split(" ").enumerate() {
            parts[i] = n.parse().unwrap();
        }

        MapEntry {
            target_start: parts[0],
            source_start: parts[1],
            range_length: parts[2],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::read_test_data;
    use std::path::Path;

    #[test]
    fn test_map_chain() {
        let test_data = read_test_data(Path::new("./example.dat")).unwrap();
        let map_chain = MapChain::from_str(&test_data);
        let seeds = get_seeds(&test_data);
        let locations: Vec<i64> = seeds.into_iter().map(|seed| map_chain.map(seed)).collect();
        let min = locations
            .iter()
            .reduce(|acc, v| if v < acc { v } else { acc })
            .unwrap();
        assert_eq!(map_chain.map(79), 82);
        println!("Locations: {:?}", locations);
        println!("  Minimum location: {}", min);
        assert_eq!(map_chain.map(14), 43);
        assert_eq!(map_chain.map(55), 86);
        assert_eq!(map_chain.map(13), 35);
    }

    #[test]
    fn test_get_seeds() {
        let test_data = read_test_data(Path::new("./example.dat")).unwrap();
        let seeds = get_seeds(&test_data);
        println!("Seeds: {:?}", seeds);
        assert_eq!(seeds.len(), 4);
        for i in (0..seeds.len()).step_by(2) {
            for j in seeds[i]..(seeds[i] + seeds[i + 1]) {
                println!("seed: {}", j);
            }
        }
    }

    #[test]
    fn test_water_map() {
        let test_data = read_test_data(Path::new("./example.dat")).unwrap();
        let maps = MapChain::from_str(&test_data).maps;
        assert_eq!(maps[2].name(), "fertilizer-to-water");
        assert_eq!(maps[2].map(53), 49);
    }

    #[test]
    fn test_named_map() {
        let test_data = read_test_data(Path::new("./example.dat")).unwrap();
        let maps = MapChain::from_str(&test_data).maps;
        assert_eq!(maps.len(), 7);
        assert_eq!(maps[1].name(), "soil-to-fertilizer");
        assert_eq!(maps[2].map_entries().len(), 4);
        assert_eq!(maps[2].map_entries()[2].target_start, 42);

        assert_eq!(maps[0].map(14), 14);
        assert_eq!(maps[0].map(79), 81);
        assert_eq!(maps[0].map(55), 57);
        assert_eq!(maps[0].map(13), 13);
    }

    #[test]
    fn test_map_entry() {
        let map = MapEntry::from("50 98 2");
        assert_eq!(
            map,
            MapEntry {
                target_start: 50,
                source_start: 98,
                range_length: 2,
            }
        );

        assert_eq!(map.map(97), 97);
        assert_eq!(map.map(98), 50);
        assert_eq!(map.map(99), 51);
        assert_eq!(map.map(100), 100);
    }
}
