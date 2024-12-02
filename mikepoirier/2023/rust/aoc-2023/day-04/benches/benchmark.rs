use input::load_file;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    day_04::part1(divan::black_box(load_file(4).unwrap()));
}

#[divan::bench]
fn part2() {
    day_04::part2(divan::black_box(load_file(4).unwrap()));
}

#[divan::bench]
fn part2_try2() {
    day_04::part2_try2(divan::black_box(load_file(4).unwrap()));
}
