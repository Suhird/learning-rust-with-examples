use std::fs;
use std::io::Error;
pub fn read_file(file_path: &str) -> Result<String, Error> {
    let content = fs::read_to_string(file_path)?;
    Ok(content)
}
