use input::load_file;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    day_02::part1(divan::black_box(load_file(2).unwrap()));
}

#[divan::bench]
fn part2() {
    day_02::part2(divan::black_box(load_file(2).unwrap()));
}
