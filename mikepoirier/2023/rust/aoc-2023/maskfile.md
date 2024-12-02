# Advent of Code 2023

## clippy (day)

> Run clippy in watch mode

```sh
crate="day-$(printf %02d $day)"
cargo watch -q -c \
    -w ${crate}/src/ \
    -x "clippy -p $crate"
```

## run (day) (part)

> Run clippy in watch mode

```sh
crate="day-$(printf %02d $day)"
cargo run -p $crate --bin part$part
```

## bench (day)

> Run clippy in watch mode

```sh
crate="day-$(printf %02d $day)"
cargo bench -p $crate
```

## profile (day) (part)

> Run clippy in watch mode

```sh
crate="day-$(printf %02d $day)"
cargo run -p $crate --bin part$part --features dhat-heap
```

## test (day)

> Run clippy in watch mode

```sh
crate="day-$(printf %02d $day)"
cargo test -p $crate
```

## new-day (day)

> Setup a new project for the given day

```sh
crate="day-$(printf %02d $day)"
echo "Creating crate $crate"
cargo new --lib $crate
cargo add -p $crate input
cargo add -p $crate dhat
cargo add -p $crate divan
cargo add -p $crate thiserror

printf "\n[[bench]]\nname = \"$crate\"\npath = \"benches/benchmark.rs\"\nharness = false" >> $crate/Cargo.toml
printf "\n\n[features]\ndhat-heap = []" >> $crate/Cargo.toml

mkdir $crate/benches

sed "s/day_01/${crate/-/_}/g;s/(1)/($day)/g" day-01/benches/benchmark.rs > $crate/benches/benchmark.rs
```
