use aocd::prelude::*;
use std::collections::HashMap;

fn encode_card(c: char) -> char {
    match c {
        'J' => '1',
        'T' => 'B',
        'Q' => 'D',
        'K' => 'E',
        'A' => 'F',
        _ => c,
    }
}

fn encode_cards(cards: &str) -> impl Iterator<Item = char> + '_ {
    cards.chars().map(encode_card)
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

    fn kind_power(&self) -> i64 {
        let mut freq: HashMap<char, i64> = HashMap::new();
        let joker = encode_card('J');

        for c in self.cards.iter().filter(|c| **c != joker) {
            *freq.entry(*c).or_insert(0) += 1;
        }

        let mut counts: Vec<i64> = freq.values().copied().collect();
        counts.sort_by(|a, b| b.cmp(a));

        let biggest_count = counts.first().copied().unwrap_or(0);
        let second_biggest_count = counts.get(1).copied().unwrap_or(0);
        let joker_count = self.cards.iter().filter(|c| **c == joker).count() as i64;

        (biggest_count + joker_count) * 10 + second_biggest_count
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

        assert_eq!(solve(puzzle), 5905);
    }

    #[test]
    fn test_kind_power_with_joker() {
        let h1 = Hand::parse("JJJJJ 0");
        let h2 = Hand::parse("JJJJA 0");
        let h3 = Hand::parse("JJJAA 0");
        let h4 = Hand::parse("JJAAA 0");
        let h5 = Hand::parse("JAAAA 0");
        let h6 = Hand::parse("AAAAA 0");

        assert_eq!(h1.kind_power(), h2.kind_power());
        assert_eq!(h2.kind_power(), h3.kind_power());
        assert_eq!(h3.kind_power(), h4.kind_power());
        assert_eq!(h4.kind_power(), h5.kind_power());
        assert_eq!(h5.kind_power(), h6.kind_power());

        let h1 = Hand::parse("JJJAK 0"); // AAAAK // Four of a kind
        let h2 = Hand::parse("JJKKA 0"); // KKKKA // Four of a kind
        let h3 = Hand::parse("JKKKA 0"); // KKKKA // Four of a kind

        assert_eq!(h1.kind_power(), h2.kind_power());
        assert_eq!(h2.kind_power(), h3.kind_power());
    }
}
