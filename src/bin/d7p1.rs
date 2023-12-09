use aocd::prelude::*;
use std::collections::HashMap;

// This encoding is better for char comparison
fn encode_cards(cards: &str) -> impl Iterator<Item = char> + '_ {
    cards.chars().map(|c| match c {
        'T' => 'B',
        'J' => 'C',
        'Q' => 'D',
        'K' => 'E',
        'A' => 'F',
        _ => c,
    })
}

struct Hand {
    cards: Vec<char>,
    bid: i64,
}

impl Hand {
    fn parse(s: &str) -> Self {
        let (cards, bid) = s.split_once(' ').expect("could not split hand in two");

        let cards = encode_cards(cards).collect();
        let bid: i64 = bid.parse().expect("bid is not a number");

        Self { cards, bid }
    }

    // AAAAA -> 50
    // AA8AA -> 41
    // 23332 -> 32
    // TTT98 -> 31
    // 23432 -> 22
    // A23A4 -> 21
    // 23456 -> 11

    fn kind_power(&self) -> i64 {
        let mut freq: HashMap<char, i64> = HashMap::new();

        for c in self.cards.iter() {
            *freq.entry(*c).or_insert(0) += 1;
        }

        let biggest_count = freq.values().copied().max().unwrap();
        let second_biggest_count = freq
            .values()
            .copied()
            .filter(|v| *v != biggest_count)
            .max()
            .unwrap_or(0);

        biggest_count * 10 + second_biggest_count
    }
}

fn solve(puzzle: &str) -> i64 {
    let mut hands: Vec<_> = puzzle.trim().lines().map(Hand::parse).collect();
    hands.sort_by(|h1, h2| {
        h1.kind_power()
            .cmp(&h2.kind_power())
            .then_with(|| h1.cards.cmp(&h2.cards))
    });

    hands
        .iter()
        .enumerate()
        .map(|(i, h)| {
            let rank = (i + 1) as i64;
            rank * h.bid
        })
        .sum()
}

#[aocd(2023, 7)]
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
        let puzzle: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

        assert_eq!(solve(puzzle), 6440);
    }

    #[test]
    fn test_hand_kind_power() {
        let full_house = Hand::parse("23332 0");
        let three_of_a_kind = Hand::parse("TTT98 0");

        assert!(full_house.kind_power() > three_of_a_kind.kind_power());
    }

    #[test]
    fn test_interpretation_sensitive_case() {
        let input = "K8KK6 75
TAK97 148
8345K 129
QT45K 170
";

        let result = 1235;
        assert_eq!(solve(input), result);
    }
}
