use std::{fs, path::Path};

mod error;
pub use error::Error;

pub fn read_test_data(path: &Path) -> Result<String, Error> {
    let data = fs::read_to_string(path)?;
    Ok(data)
}
