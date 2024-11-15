use std::{path::Path};
use common::{read_test_data, Error};
use schematic::{Gear, PartNumber, Schematic, TokenType};

mod schematic;

fn main() -> Result<(), Error> {
    // Example
    let example_data = read_test_data(Path::new("./day03/example.dat"))?;
    let schematic = Schematic::from_string(&example_data);

    let puzzle_result = schematic
        .part_numbers
        .iter()
        .map(|pn| pn.0.value())
        .reduce(|acc, v| acc + v)
        .unwrap();
    println!("Example: Sum of part numbers: {}", puzzle_result);

    let sum_gear_ratios = schematic.gears.iter().map(|gear| gear.ratio()).reduce(|acc, v| acc + v).unwrap();
    println!("Example: Sum of gear ratios: {}", sum_gear_ratios);

    // Test
    let test_data = read_test_data(Path::new("./day03/test.dat"))?;
    let schematic = Schematic::from_string(&test_data);

    let puzzle_result = schematic
        .part_numbers
        .iter()
        .map(|pn| pn.0.value())
        .reduce(|acc, v| acc + v)
        .unwrap();
    println!("Test: Sum of part numbers: {}", puzzle_result);

    let sum_gear_ratios = schematic.gears.iter().map(|gear| gear.ratio()).reduce(|acc, v| acc + v).unwrap();
    println!("Test: Sum of gear ratios: {}", sum_gear_ratios);
    Ok(())
}
