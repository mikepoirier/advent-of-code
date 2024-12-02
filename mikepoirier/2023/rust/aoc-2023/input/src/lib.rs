use std::path::PathBuf;

pub use error::{Error, Result};

mod error;

pub fn save_input(day: u32, input: impl AsRef<[u8]>) -> Result<()> {
    let file = input_file(day)?;

    std::fs::write(file, input.as_ref())?;

    Ok(())
}

pub fn load_file(day: u32) -> Result<String> {
    let file = input_file(day)?;

    let input = std::fs::read_to_string(file)?;

    Ok(input)
}

fn input_file(day: u32) -> Result<PathBuf> {
    let mut data_dir = dirs::data_dir().ok_or_else(|| Error::NoDataDir)?;
    data_dir.push("aoc-2023");

    if !data_dir.exists() {
        std::fs::create_dir_all(&data_dir)?;
    }

    data_dir.push(format!("day_{:02}.txt", day));

    Ok(data_dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_file_works() {
        let data_dir = dirs::data_dir().unwrap().join("aoc-2023");
        let file = input_file(1).unwrap();

        assert_eq!(file, data_dir.join("day_01.txt"));

        let file = input_file(21).unwrap();
        assert_eq!(file, data_dir.join("day_21.txt"));
    }
}
