pub fn load_input() -> String {
    std::fs::read_to_string("./input.txt")
        .expect("I have a fever. And the only prescription is... MORE INPUT!")
}

pub fn part1(input: impl AsRef<str>) -> String {
    let input = input.as_ref();

    let mut elf_calories = vec![];
    let mut current_elf = vec![];

    for line in input.lines() {
        if line.is_empty() {
            elf_calories.push(current_elf.clone());
            current_elf.clear();
        } else {
            current_elf.push(line.parse::<u32>().unwrap())
        }
    }

    elf_calories
        .into_iter()
        .map(|v| v.iter().sum())
        .max()
        .map(|v: u32| format!("{v}"))
        .unwrap_or_default()
}

pub fn part2(input: impl AsRef<str>) -> String {
    let _input = input.as_ref();
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"#;

    #[test]
    fn part1_works() {
        let expected = "24000";

        let actual = part1(INPUT);

        assert_eq!(actual, expected);
    }
}
