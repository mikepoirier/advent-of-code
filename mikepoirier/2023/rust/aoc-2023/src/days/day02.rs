use std::{convert::identity, iter::Sum, ops::Add};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, u64 as Nomu64},
    combinator::{map, value},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

use crate::{Error, Result, Solution, SolutionOutput};

pub struct Day02;

impl Solution for Day02 {
    fn solve(&self, input: impl AsRef<str>) -> Result<SolutionOutput> {
        let input = input.as_ref();

        let (_, games) = games(input).map_err(|e| Error::Nom(format!("{e}")))?;

        let possible_games: Vec<_> = games.iter().filter(|g| g.possible(12, 13, 14)).collect();

        let part1 = possible_games.iter().map(|g| g.id).sum::<u64>();
        let part2: u64 = games.iter().map(Game::power).sum();

        Ok(SolutionOutput {
            part1: Some(format!("{part1}")),
            part2: Some(format!("{part2}")),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    pub id: u64,
    pub min_red: u64,
    pub min_green: u64,
    pub min_blue: u64,
}

impl Game {
    fn new(id: u64, rounds: Vec<Round>) -> Self {
        let (total_red, total_green, total_blue) = rounds
            .into_iter()
            .map(|r| (r.red, r.green, r.blue))
            .fold((0, 0, 0), |acc, next| {
                let red = acc.0.max(next.0);
                let green = acc.1.max(next.1);
                let blue = acc.2.max(next.2);
                (red, green, blue)
            });
        Self {
            id,
            min_red: total_red,
            min_green: total_green,
            min_blue: total_blue,
        }
    }

    fn possible(&self, red: u64, green: u64, blue: u64) -> bool {
        self.min_red <= red && self.min_green <= green && self.min_blue <= blue
    }

    fn power(&self) -> u64 {
        self.min_red * self.min_green * self.min_blue
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
struct Round {
    pub red: u64,
    pub green: u64,
    pub blue: u64,
}

impl Round {
    fn new(red: u64, green: u64, blue: u64) -> Self {
        Self { red, green, blue }
    }
}

impl Add for Round {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl Add<&Round> for Round {
    type Output = Self;

    fn add(self, rhs: &Round) -> Self::Output {
        Self {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl Sum for Round {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|acc, next| acc + next).unwrap_or_default()
    }
}

#[derive(Debug, Clone)]
enum Color {
    Red,
    Green,
    Blue,
}

fn game_id(str: &str) -> IResult<&str, u64> {
    terminated(
        preceded(tag("Game "), nom::character::complete::u64),
        tag(":"),
    )(str)
}

// " 3 blue"
fn partial_round(str: &str) -> IResult<&str, Round> {
    map(
        separated_pair(
            preceded(tag(" "), Nomu64),
            tag(" "),
            alt((
                value(Color::Red, tag("red")),
                value(Color::Green, tag("green")),
                value(Color::Blue, tag("blue")),
            )),
        ),
        |(count, color)| match color {
            Color::Red => Round::new(count, 0, 0),
            Color::Green => Round::new(0, count, 0),
            Color::Blue => Round::new(0, 0, count),
        },
    )(str)
}

// " 3 blue, 4 red"
fn round(str: &str) -> IResult<&str, Round> {
    map(separated_list1(tag(","), partial_round), |partial_rounds| {
        partial_rounds.into_iter().sum()
    })(str)
}

// " 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
fn rounds(str: &str) -> IResult<&str, Vec<Round>> {
    separated_list1(tag(";"), round)(str)
}

// "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
fn game(str: &str) -> IResult<&str, Game> {
    let (remainder, (game_id, rounds)) = tuple((game_id, rounds))(str)?;

    Ok((remainder, Game::new(game_id, rounds)))
}

fn games(str: &str) -> IResult<&str, Vec<Game>> {
    separated_list1(line_ending, game)(str)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#;

    #[test]
    fn part1_works() {
        let solution = Day02.solve(INPUT).unwrap();

        assert_eq!(solution.part1, Some("8".into()))
    }

    #[test]
    fn part2_works() {
        let solution = Day02.solve(INPUT).unwrap();

        assert_eq!(solution.part2, Some("2286".into()))
    }

    #[test]
    fn game_works() {
        let line1 = INPUT.lines().next().unwrap();

        let (_, game) = game(line1).unwrap();

        assert_eq!(
            game,
            Game::new(
                1,
                vec![
                    Round::new(4, 0, 3),
                    Round::new(1, 2, 6),
                    Round::new(0, 2, 0)
                ]
            )
        )
    }

    #[test]
    fn games_works() {
        let (rem, games) = games(INPUT).unwrap();

        assert_eq!(
            games[0..=1],
            vec![
                Game::new(
                    1,
                    vec![
                        Round::new(4, 0, 3),
                        Round::new(1, 2, 6),
                        Round::new(0, 2, 0)
                    ]
                ),
                Game::new(
                    2,
                    vec![
                        Round::new(0, 2, 1),
                        Round::new(1, 3, 4),
                        Round::new(0, 1, 1)
                    ]
                ),
            ]
        )
    }
}
