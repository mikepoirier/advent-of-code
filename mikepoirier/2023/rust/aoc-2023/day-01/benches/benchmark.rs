use input::load_file;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    day_01::part1(divan::black_box(load_file(1).unwrap()));
}

#[divan::bench]
fn part2() {
    day_01::part2(divan::black_box(load_file(1).unwrap()));
}
