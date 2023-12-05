use aocd::prelude::*;

#[aocd(2023, 1)]
fn main() {
    let result: u32 = input!()
        .lines()
        .map(|l| {
            let mut iter = l.chars().filter_map(|c| c.to_digit(10));

            let first = iter.next().unwrap();
            let last = iter.last().unwrap_or(first);

            first * 10 + last
        })
        .sum();

    println!("{}", result);
}
