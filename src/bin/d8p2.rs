// The idea of using LCM came from Reddit memes :P
// My original brute force solution that looks like part 1 didn't finish after
// a long time so I had to discard it.

use aocd::prelude::*;
use std::collections::HashMap;

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u128, b: u128) -> u128 {
    (a / gcd(a, b)) * b
}

fn normalize(puzzle: &str) -> String {
    puzzle
        .replace("\n\n", "\n")
        .replace([' ', '(', ')'], "")
        .replace('=', ",")
}

fn follow(network: &HashMap<&str, (&str, &str)>, directions: &str, node_id: &str) -> u128 {
    let mut directions = directions.chars().cycle();

    let mut node_id = node_id;
    let mut steps: u128 = 0;

    while !node_id.ends_with('Z') {
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

fn solve(puzzle: &str) -> u128 {
    let puzzle = normalize(puzzle);

    let directions = puzzle.lines().next().expect("no directions to read");

    let network: HashMap<&str, (&str, &str)> = puzzle
        .lines()
        .skip(1)
        .map(|line| {
            let (from, to) = line.split_once(',').expect("can not split node");
            let (left, right) = to.split_once(',').expect("can not split node paths");
            (from, (left, right))
        })
        .collect();

    network
        .keys()
        .filter(|id| id.ends_with('A'))
        .copied()
        .map(|id| follow(&network, directions, id))
        .reduce(lcm)
        .expect("nothing to reduce")
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

    const LONG_EXAMPLE: &str = "LR

11A = (11B, FFF)
11B = (FFF, 11Z)
11Z = (11B, FFF)
22A = (22B, FFF)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
FFF = (FFF, FFF)
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
        assert_eq!(solve(LONG_EXAMPLE), 6);
    }
}
