use std::collections::HashMap;

use aocd::prelude::*;

fn normalize(puzzle: &str) -> String {
    puzzle
        .replace("\n\n", "\n")
        .replace([' ', '(', ')'], "")
        .replace('=', ",")
}

fn solve(puzzle: &str) -> i64 {
    let puzzle = normalize(puzzle);

    let mut directions = puzzle
        .lines()
        .next()
        .expect("no directions to read")
        .chars()
        .cycle();

    let network: HashMap<&str, (&str, &str)> = puzzle
        .lines()
        .skip(1)
        .map(|line| {
            let (from, to) = line.split_once(',').expect("can not split node");
            let (left, right) = to.split_once(',').expect("can not split node paths");
            (from, (left, right))
        })
        .collect();

    let mut node_id = "AAA";
    let mut steps: i64 = 0;

    while node_id != "ZZZ" {
        let node_paths = network.get(node_id).expect("node id not found");
        let dir = directions.next().expect("can not read next direction");

        node_id = match dir {
            'L' => node_paths.0,
            'R' => node_paths.1,
            _ => panic!("invalid direction"),
        };

        steps += 1;
    }

    steps
}

#[aocd(2023, 8)]
fn main() {
    let input = input!();
    let result = solve(&input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SHORT_EXAMPLE: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

    const LONG_EXAMPLE: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

    #[test]
    fn test_normalize() {
        let expected: &str = "LLR
AAA,BBB,BBB
BBB,AAA,ZZZ
ZZZ,ZZZ,ZZZ
";
        assert_eq!(normalize(SHORT_EXAMPLE), expected);
    }

    #[test]
    fn test_solve_against_long_example() {
        assert_eq!(solve(LONG_EXAMPLE), 2);
    }

    #[test]
    fn test_solve_against_short_example() {
        assert_eq!(solve(SHORT_EXAMPLE), 6);
    }
}
