fn main() {
    let result: u32 = std::fs::read_to_string("inputs/d1.txt")
        .expect("Unable to read input")
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
