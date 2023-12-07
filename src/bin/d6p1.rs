use aocd::prelude::*;

// did some random math while in the toilet and validated with geogebra

// f(x)=-x^(2)+7 x
// f(x)=-x^(2)+tx

// -(x**2)+tx-r
// t = the time limit
// r = the record distance

fn bhaskara_equation(a: f64, b: f64, c: f64) -> (f64, f64) {
    let right = (b.powf(2.0) - 4.0 * a * c).sqrt();
    let down = 2.0 * a;

    let x1 = (-b + right) / down;
    let x2 = (-b - right) / down;
    (x1, x2)
}

#[derive(Debug, PartialEq, Eq)]
struct Race {
    time_limit: i64,
    record_distance: i64,
}

impl Race {
    fn new(time_limit: i64, record_distance: i64) -> Self {
        Self {
            time_limit,
            record_distance,
        }
    }

    fn compute_winner_solutions(&self) -> std::ops::RangeInclusive<i64> {
        let Race {
            time_limit,
            record_distance,
        } = self;

        let a = -1.0;
        let b = *time_limit as f64;
        // -1 because I don't want to tie, I want to win.
        let c = -*record_distance as f64 - 1.0;

        let (x1, x2) = bhaskara_equation(a, b, c);

        let min = x1.min(x2).ceil() as i64;
        let max = x1.max(x2).floor() as i64;

        min..=max
    }
}

fn parse(puzzle: &str) -> impl Iterator<Item = Race> + '_ {
    let mut parsed_lines = puzzle.lines().map(|l| {
        l.split_once(':')
            .unwrap()
            .1
            .split_whitespace()
            .map(|n| n.parse::<i64>().expect("could not parse number from input"))
    });

    let times_iter = parsed_lines.next().expect("can not read times line");
    let records_iter = parsed_lines.next().expect("can not read records line");

    times_iter.zip(records_iter).map(|(t, r)| Race::new(t, r))
}

fn solve(puzzle: &str) -> i64 {
    parse(puzzle)
        .map(|r| r.compute_winner_solutions().count() as i64)
        .reduce(|acc, n| acc * n)
        .expect("no solutions count to reduce")
}

#[aocd(2023, 6)]
fn main() {
    let input = input!();
    let result = solve(&input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PUZZLE: &str = "Time:      7  15   30
Distance:  9  40  200
";

    #[test]
    fn test_parse() {
        let expected = vec![Race::new(7, 9), Race::new(15, 40), Race::new(30, 200)];
        assert_eq!(parse(EXAMPLE_PUZZLE).collect::<Vec<_>>(), expected);
    }

    #[test]
    fn test_solve_against_example() {
        assert_eq!(solve(EXAMPLE_PUZZLE), 288);
    }

    #[test]
    fn test_compute_winner_solutions() {
        assert_eq!(
            Race::new(7, 9)
                .compute_winner_solutions()
                .collect::<Vec<_>>(),
            vec![2, 3, 4, 5]
        );

        assert_eq!(Race::new(15, 40).compute_winner_solutions().count(), 8);
        assert_eq!(Race::new(30, 200).compute_winner_solutions().count(), 9);
    }
}
