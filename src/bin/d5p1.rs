use aocd::prelude::*;

fn normalize(puzzle: &str) -> String {
    puzzle
        .trim()
        .replacen("seeds: ", "", 1)
        .lines()
        .filter(|l| match l.chars().next() {
            Some(c) => c.is_ascii_digit(),
            None => true,
        })
        .collect::<Vec<_>>()
        .join("\n")
}

struct RangeMapper {
    source_start: i64,
    source_end: i64,
    target_start: i64,
    target_end: i64,
}

impl RangeMapper {
    fn new(source_start: i64, target_start: i64, range_length: i64) -> Self {
        Self {
            source_start,
            target_start,
            source_end: source_start + range_length,
            target_end: target_start + range_length,
        }
    }

    fn parse(s: &str) -> Self {
        let mut iter = s.split(' ');

        let target_start = iter
            .next()
            .expect("no target start to read")
            .parse()
            .expect("target start is not a number");

        let source_start = iter
            .next()
            .expect("no source start to read")
            .parse()
            .expect("source start is not a number");

        let range_length = iter
            .next()
            .expect("no range length to read")
            .parse()
            .expect("range length is not a number");

        Self::new(source_start, target_start, range_length)
    }

    fn source_range(&self) -> std::ops::Range<i64> {
        self.source_start..self.source_end
    }

    fn target_range(&self) -> std::ops::Range<i64> {
        self.target_start..self.target_end
    }

    fn map(&self, source: i64) -> Option<i64> {
        if !self.source_range().contains(&source) {
            return None;
        }

        let delta = source - self.source_start;
        let result = self.target_start + delta;

        if !self.target_range().contains(&result) {
            return None;
        }

        Some(result)
    }
}

struct CompositeRangeMapper {
    mappers: Vec<RangeMapper>,
}

impl CompositeRangeMapper {
    fn parse(s: &str) -> Self {
        Self {
            mappers: s.lines().map(RangeMapper::parse).collect(),
        }
    }

    fn map(&self, source: i64) -> Option<i64> {
        self.mappers.iter().filter_map(|m| m.map(source)).next()
    }
}

fn solve(puzzle: &str) -> i64 {
    let puzzle = normalize(puzzle);
    let mut iter = puzzle.split("\n\n");

    let seeds: Vec<i64> = iter
        .next()
        .expect("no seed list input")
        .split(' ')
        .map(|n| n.parse().expect("seed number is not a number"))
        .collect();

    let mappers: Vec<_> = iter.map(CompositeRangeMapper::parse).collect();

    seeds
        .iter()
        .map(|s| mappers.iter().fold(*s, |acc, m| m.map(acc).unwrap_or(acc)))
        .min()
        .expect("no seeds to calculate the min")
}

#[aocd(2023, 5)]
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
        let puzzle: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15
";

        let expected: &str = "79 14 55 13

50 98 2
52 50 48

0 15 37
37 52 2
39 0 15";
        assert_eq!(normalize(puzzle), expected);
    }

    #[test]
    fn test_solve_against_example() {
        let puzzle: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

        assert_eq!(solve(puzzle), 35);
    }

    #[test]
    fn test_range_mapper_map() {
        let mapper = RangeMapper::new(98, 50, 2);
        assert_eq!(mapper.map(97), None);
        assert_eq!(mapper.map(98), Some(50));
        assert_eq!(mapper.map(99), Some(51));
        assert_eq!(mapper.map(100), None);
    }
}
