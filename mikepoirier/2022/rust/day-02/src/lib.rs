use std::str::FromStr;

pub fn load_input() -> String {
    std::fs::read_to_string("./input.txt")
        .expect("I have a fever. And the only prescription is... MORE INPUT!")
}

enum Move {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(format!("{s} is not a valid move")),
        }
    }
}

pub fn part1(input: impl AsRef<str>) -> String {
    let input = input.as_ref();
    let mut score = 0;
    for line in input.lines() {
        let moves: (Move, Move) = line
            .split_once(" ")
            .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
            .unwrap();

        let round = match moves {
            (Move::Rock, Move::Rock) => 1 + 3,
            (Move::Rock, Move::Paper) => 2 + 6,
            (Move::Rock, Move::Scissors) => 3 + 0,
            (Move::Paper, Move::Rock) => 1 + 0,
            (Move::Paper, Move::Paper) => 2 + 3,
            (Move::Paper, Move::Scissors) => 3 + 6,
            (Move::Scissors, Move::Rock) => 1 + 6,
            (Move::Scissors, Move::Paper) => 2 + 0,
            (Move::Scissors, Move::Scissors) => 3 + 3,
        };

        score += round;
    }
    format!("{score}")
}

pub fn part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"A Y
B X
C Z
"#;

    #[test]
    fn part1_works() {
        let expected = "15";

        let actual = part1(INPUT);

        assert_eq!(actual, expected);
    }

    #[test]
    fn part2_works() {
        let expected = "12";

        let actual = part2(INPUT);

        assert_eq!(actual, expected);
    }
}
