use std::{error::Error, fs, path::Path};

pub fn read_test_data(path: &Path) -> Result<String, Box<dyn Error>> {
    let data = fs::read_to_string(path)?;
    Ok(data)
}

