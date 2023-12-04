use std::{fmt::Display, str::FromStr};

use clap::Subcommand;

use days::{Day01, Day02};
pub use error::{Error, Result};
use input::{load_input, save_input};

mod days;
mod error;
mod input;

#[derive(Debug, Subcommand)]
pub enum Command {
    Import { day: Day },
    Solve { day: Day },
}

#[derive(Debug, Clone)]
pub enum Day {
    Day01,
    Day02,
    Day03,
    Day04,
    Day05,
    Day06,
    Day07,
    Day08,
    Day09,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
    Day17,
    Day18,
    Day19,
    Day20,
    Day21,
    Day22,
    Day23,
    Day24,
    Day25,
}

impl Day {
    pub fn day_str(&self) -> &str {
        match self {
            Day::Day01 => "01",
            Day::Day02 => "02",
            Day::Day03 => "03",
            Day::Day04 => "04",
            Day::Day05 => "05",
            Day::Day06 => "06",
            Day::Day07 => "07",
            Day::Day08 => "08",
            Day::Day09 => "09",
            Day::Day10 => "10",
            Day::Day11 => "11",
            Day::Day12 => "12",
            Day::Day13 => "13",
            Day::Day14 => "14",
            Day::Day15 => "15",
            Day::Day16 => "16",
            Day::Day17 => "17",
            Day::Day18 => "18",
            Day::Day19 => "19",
            Day::Day20 => "20",
            Day::Day21 => "21",
            Day::Day22 => "22",
            Day::Day23 => "23",
            Day::Day24 => "24",
            Day::Day25 => "25",
        }
    }
}

impl FromStr for Day {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "1" => Ok(Day::Day01),
            "2" => Ok(Day::Day02),
            "3" => Ok(Day::Day03),
            "4" => Ok(Day::Day04),
            "5" => Ok(Day::Day05),
            "6" => Ok(Day::Day06),
            "7" => Ok(Day::Day07),
            "8" => Ok(Day::Day08),
            "9" => Ok(Day::Day09),
            "10" => Ok(Day::Day10),
            "11" => Ok(Day::Day11),
            "12" => Ok(Day::Day12),
            "13" => Ok(Day::Day13),
            "14" => Ok(Day::Day14),
            "15" => Ok(Day::Day15),
            "16" => Ok(Day::Day16),
            "17" => Ok(Day::Day17),
            "18" => Ok(Day::Day18),
            "19" => Ok(Day::Day19),
            "20" => Ok(Day::Day20),
            "21" => Ok(Day::Day21),
            "22" => Ok(Day::Day22),
            "23" => Ok(Day::Day23),
            "24" => Ok(Day::Day24),
            "25" => Ok(Day::Day25),
            _ => Err(Error::InvalidDay),
        }
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.day_str();

        write!(f, "Day {s}")
    }
}

impl Solution for Day {
    fn solve(&self, input: impl AsRef<str>) -> Result<SolutionOutput> {
        match self {
            Day::Day01 => Day01.solve(input),
            Day::Day02 => Day02.solve(input),
            Day::Day03 => todo!(),
            Day::Day04 => todo!(),
            Day::Day05 => todo!(),
            Day::Day06 => todo!(),
            Day::Day07 => todo!(),
            Day::Day08 => todo!(),
            Day::Day09 => todo!(),
            Day::Day10 => todo!(),
            Day::Day11 => todo!(),
            Day::Day12 => todo!(),
            Day::Day13 => todo!(),
            Day::Day14 => todo!(),
            Day::Day15 => todo!(),
            Day::Day16 => todo!(),
            Day::Day17 => todo!(),
            Day::Day18 => todo!(),
            Day::Day19 => todo!(),
            Day::Day20 => todo!(),
            Day::Day21 => todo!(),
            Day::Day22 => todo!(),
            Day::Day23 => todo!(),
            Day::Day24 => todo!(),
            Day::Day25 => todo!(),
        }
    }
}

pub struct SolutionOutput {
    pub part1: Option<String>,
    pub part2: Option<String>,
}

pub trait Solution {
    fn solve(&self, input: impl AsRef<str>) -> Result<SolutionOutput>;
}

pub fn run(command: Command) -> Result<()> {
    match command {
        Command::Import { day } => import(day)?,
        Command::Solve { day } => solve(day)?,
    }

    Ok(())
}

fn import(day: Day) -> Result<()> {
    let stdin = std::io::stdin().lock();
    let input = std::io::read_to_string(stdin)?;
    save_input(&day, input)?;
    Ok(())
}

fn solve(day: Day) -> Result<()> {
    let input = load_input(&day)?;
    let output = day.solve(input)?;
    let part1 = output
        .part1
        .as_ref()
        .map(String::as_ref)
        .unwrap_or("Not completed");
    let part2 = output
        .part2
        .as_ref()
        .map(String::as_ref)
        .unwrap_or("Not completed");
    println!("Results for Day {day}:\n\tPart 1: {part1}\n\tPart 2: {part2}");
    Ok(())
}
