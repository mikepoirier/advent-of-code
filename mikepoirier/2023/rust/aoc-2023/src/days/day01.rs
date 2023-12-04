use crate::{Error, Result, Solution, SolutionOutput};

pub struct Day01;

impl Solution for Day01 {
    fn solve(&self, input: impl AsRef<str>) -> Result<SolutionOutput> {
        let input = input.as_ref();

        Ok(SolutionOutput {
            part1: Some(part1(input)),
            part2: Some(part2(input)),
        })
    }
}

fn part1(input: &str) -> String {
    let result: u32 = input
        .lines()
        .map(|line| {
            let first = line
                .chars()
                .find(|c| c.is_numeric())
                .ok_or(Error::InputProcessing)?;
            let last = line
                .chars()
                .rev()
                .find(|c| c.is_numeric())
                .ok_or(Error::InputProcessing)?;
            Ok(format!("{first}{last}"))
        })
        .filter_map(Result::ok)
        .map(|s| Ok(s.parse::<u32>()?))
        .filter_map(Result::ok)
        .sum();
    format!("{}", result)
}

fn part2(input: &str) -> String {
    let input = input
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine");

    part1(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"#;

    static INPUT_2: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"#;

    #[test]
    fn part1_works() {
        let solution = Day01.solve(INPUT).unwrap();

        assert_eq!(solution.part1, Some("142".into()))
    }

    #[test]
    fn part2_works() {
        let solution = Day01.solve(INPUT_2).unwrap();

        assert_eq!(solution.part2, Some("281".into()))
    }
}
