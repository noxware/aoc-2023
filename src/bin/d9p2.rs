use aocd::prelude::*;

fn predict(current: &[i64]) -> i64 {
    if current.iter().all(|n| *n == 0) {
        return 0;
    }

    let next = current.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
    current[0] - predict(&next)
}

fn solve(puzzle: &str) -> i64 {
    puzzle
        .trim()
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse().expect("not a number"))
                .collect::<Vec<_>>()
        })
        .map(|history| predict(&history))
        .sum()
}

#[aocd(2023, 9)]
fn main() {
    let input = input!();
    let result = solve(&input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_against_example() {
        let puzzle: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

        assert_eq!(solve(puzzle), 2);
    }
}
