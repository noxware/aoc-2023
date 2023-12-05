# 🌟🦀 aoc-2023 🦀🌟

## About

This repo contains my solutions for the [Advent of Code 2023](https://adventofcode.com/2023) written in Rust.

## Repo format example

- `d1p1.rs` is my main solution for day 1 part 1.
- `d1p2a1.rs` is another (alternative 1) solution for day 1 part 2.

## Running a solution

To run a solution use `cargo run --bin d1p1` for day 1 part 1.

## Providing the puzzle input

Puzzle inputs are fetched automatically thanks to the [aocd](https://docs.rs/aocd/latest/aocd/)
crate. This is to avoid publishing inputs [as requested by the author of AoC](https://www.reddit.com/r/adventofcode/comments/18an94z/psa_dont_share_your_inputs_even_in_your_github/).

Follow the instructions of `aocd` to set up your session token, necessary for
fetching your inputs.

## Tests

To run tests use `cargo test`. For each day part, at least the public example
is tested but this don't cover edge cases. For debugging purposes or without
a particular reason there may be more tests.
