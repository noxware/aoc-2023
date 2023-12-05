fn normalize(puzzle: &str) -> String {
    todo!();
}

fn solve(puzzle: &str) -> u32 {
    let puzzle = normalize(puzzle);
    todo!();
}

fn main() {
    let input = std::fs::read_to_string("inputs/dN.txt").expect("Unable to read input");
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
