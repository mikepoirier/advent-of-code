pub use error::{Error, Result};

mod error;

pub fn part1(input: impl AsRef<str>) -> String {
    let input = input.as_ref();
    let result: u32 = input
        .lines()
        .map(|line| {
            let first = line
                .chars()
                .find(|c| c.is_numeric())
                .ok_or(Error::CharNotFound)?;
            let last = line
                .chars()
                .rev()
                .find(|c| c.is_numeric())
                .ok_or(Error::CharNotFound)?;
            Ok(format!("{first}{last}"))
        })
        .filter_map(Result::ok)
        .map(|s| Ok(s.parse::<u32>()?))
        .filter_map(Result::ok)
        .sum();
    format!("{}", result)
}

pub fn part2(input: impl AsRef<str>) -> String {
    let input = input
        .as_ref()
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine");

    part1(input)
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
        let part1 = part1(INPUT);

        assert_eq!(part1, "142")
    }

    #[test]
    fn part2_works() {
        let part2 = part2(INPUT_2);

        assert_eq!(part2, "281")
    }
}
