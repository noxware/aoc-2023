// Simplier than my original solution (now d1p2a1) and preserves everything
// from part 1, but may be less performant.

fn main() {
    let result: u32 = std::fs::read_to_string("inputs/d1.txt")
        .expect("Unable to read input")
        .lines()
        .map(|l| {
            l.replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine")
        })
        .map(|l| {
            let mut iter = l.chars().filter_map(|c| c.to_digit(10));

            let first = iter.next().unwrap();
            let last = iter.last().unwrap_or(first);

            first * 10 + last
        })
        .sum();

    println!("{}", result);
}
