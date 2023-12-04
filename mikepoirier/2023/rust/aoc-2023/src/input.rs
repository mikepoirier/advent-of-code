use std::path::PathBuf;

use crate::{Day, Error, Result};

pub fn save_input(day: &Day, input: String) -> Result<()> {
    let input_file = input_file(day)?;
    std::fs::write(&input_file, input)?;
    println!("Saved input to {input_file:?}");
    Ok(())
}

pub fn load_input(day: &Day) -> Result<String> {
    let input_file = input_file(day)?;
    println!("Reading input from {input_file:?}");
    let input = std::fs::read_to_string(input_file)?;
    Ok(input)
}

fn input_file(day: &Day) -> Result<PathBuf> {
    let mut input_dir = dirs::data_dir().ok_or_else(|| Error::NoSaveLocation)?;
    input_dir.push("aoc-2023");
    if !input_dir.exists() {
        std::fs::create_dir_all(&input_dir)?;
    }
    Ok(input_dir.join(format!("day_{}.txt", day.day_str())))
}
