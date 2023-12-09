use aocd::prelude::*;

fn normalize(puzzle: &str) -> String {
    todo!();
}

fn solve(puzzle: &str) -> i64 {
    let puzzle = normalize(puzzle);
    todo!();
}

#[aocd(2023, 0)]
fn main() {
    let input = input!();
    let result = solve(&input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize() {
        let puzzle: &str = todo!();
        let expected: &str = todo!();
        assert_eq!(normalize(puzzle), expected);
    }

    #[test]
    fn test_solve_against_example() {
        let puzzle: &str = "Line 1
Line 2
";

        assert_eq!(solve(puzzle), todo!());
    }
}
