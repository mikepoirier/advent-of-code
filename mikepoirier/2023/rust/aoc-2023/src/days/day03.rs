use std::collections::BTreeSet;

use crate::{Result, Solution, SolutionOutput};

pub struct Day03;

impl Solution for Day03 {
    fn solve(&self, input: impl AsRef<str>) -> Result<SolutionOutput> {
        let input = input.as_ref();

        let schematic = Schematic::new(input);

        let parts = schematic.part_numbers();
        let gears = schematic.gears();

        let part1: u32 = parts.iter().map(Part::number).sum();
        let part2: u32 = gears.iter().map(Gear::ratio).sum();

        Ok(SolutionOutput {
            part1: Some(format!("{part1}")),
            part2: Some(format!("{part2}")),
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct Point(usize, usize);

impl Point {
    fn x(&self) -> usize {
        self.0
    }

    fn y(&self) -> usize {
        self.1
    }
}

struct Symbol {
    symbol: char,
    point: Point,
}

impl Symbol {
    fn new(symbol: char, point: Point) -> Self {
        Self { symbol, point }
    }
}

#[derive(Debug, Clone)]
struct Box(Point, Point);

impl Box {
    fn contains(&self, point: &Point) -> bool {
        let x_range = self.0.x()..=self.1.x();
        let y_range = self.0.y()..=self.1.y();

        x_range.contains(&point.x()) && y_range.contains(&point.y())
    }
}

#[derive(Debug, Clone)]
#[allow(unused)]
struct Part {
    number: u32,
    start: Point,
    area: Box,
}

impl Part {
    fn new(number: impl Into<String>, start: Point) -> Result<Self> {
        let number = number.into();

        let area = Box(
            Point(start.x().saturating_sub(1), start.y().saturating_sub(1)),
            Point(
                start.x().saturating_add(number.len()),
                start.y().saturating_add(1),
            ),
        );
        let number = number.parse()?;
        Ok(Self {
            number,
            start,
            area,
        })
    }

    fn number(&self) -> u32 {
        self.number
    }

    fn is_adjacent(&self, point: &Point) -> bool {
        self.area.contains(point)
    }
}

struct Gear(Part, Part);

impl Gear {
    fn ratio(&self) -> u32 {
        println!("G1: {}, G2: {}", self.0.number(), self.1.number());
        self.0.number() * self.1.number()
    }
}

struct Schematic {
    data: String,
}

impl Schematic {
    fn new(data: impl Into<String>) -> Self {
        let data = data.into();
        Self { data }
    }

    fn symbol_chars(&self) -> BTreeSet<char> {
        self.data
            .chars()
            .filter(|c| !c.is_numeric())
            .filter(|c| !c.is_whitespace())
            .filter(|c| !c.is_lowercase())
            .filter(|c| !c.is_uppercase())
            .filter(|c| c != &'.')
            .collect()
    }

    fn components(&self) -> (Vec<Part>, Vec<Symbol>) {
        let data = &self.data;
        let mut parts = vec![];
        let symbol_chars = self.symbol_chars();
        let mut symbols = vec![];
        let mut last_number_start = Point(0, 0);
        let mut number = String::new();

        for (y, line) in data.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                let point = Point(x, y);
                if last_number_start.y() < y && !number.is_empty() {
                    parts.push(Part::new(number, last_number_start).unwrap());
                    number = String::new();
                }

                if char.is_numeric() {
                    if number.is_empty() {
                        last_number_start = point
                    }
                    number.push(char);
                    continue;
                }

                if !number.is_empty() {
                    parts.push(Part::new(number, last_number_start).unwrap());
                    number = String::new();
                }

                if symbol_chars.contains(&char) {
                    let symbol = Symbol::new(char, point);
                    symbols.push(symbol);
                }
            }
        }

        (parts, symbols)
    }

    fn part_numbers(&self) -> Vec<Part> {
        let (parts, symbol_points) = self.components();

        parts
            .into_iter()
            .filter(|n| symbol_points.iter().any(|s| n.is_adjacent(&s.point)))
            .collect()
    }

    fn gears(&self) -> Vec<Gear> {
        let (parts, symbols) = self.components();

        let mut gears = vec![];

        for symbol in symbols {
            if symbol.symbol != '*' {
                continue;
            }

            let parts: Vec<_> = parts
                .iter()
                .filter(|p| p.is_adjacent(&symbol.point))
                .collect();

            if parts.len() == 2 {
                gears.push(Gear(parts[0].clone(), parts[1].clone()))
            }
        }

        gears
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..
        */
    static INPUT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    #[test]
    fn part1_works() {
        let result = Day03.solve(INPUT).unwrap();

        assert_eq!(result.part1, Some("4361".into()))
    }

    #[test]
    fn part2_works() {
        let result = Day03.solve(INPUT).unwrap();

        assert_eq!(result.part2, Some("467835".into()))
    }

    #[test]
    fn symbols_works() {
        let expected = BTreeSet::from(['*', '#', '+', '$']);
        let schematic = Schematic::new(format!("{INPUT}aA"));

        let symbols = schematic.symbol_chars();

        assert_eq!(symbols, expected)
    }
}
