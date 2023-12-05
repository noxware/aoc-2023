// This solution is not efficient. It uses more than a GB and it is very slow
// if you don't run it in release mode.

use aocd::prelude::*;

fn normalize(puzzle: &str) -> String {
    return puzzle
        .trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
        .replace("Card ", "")
        .replace(": ", "|")
        .replace(" | ", "|");
}

#[derive(Debug, Clone)]
struct Card {
    id: i32,
    winning_numbers: Vec<i32>,
    own_numbers: Vec<i32>,
}

impl Card {
    fn parse(s: &str) -> Self {
        let mut iter = s.split('|');

        let id: i32 = iter.next().unwrap().parse().unwrap();
        let winning_numbers: Vec<i32> = iter
            .next()
            .unwrap()
            .split(' ')
            .map(|s| s.parse().unwrap())
            .collect();
        let own_numbers: Vec<i32> = iter
            .next()
            .unwrap()
            .split(' ')
            .map(|s| s.parse().unwrap())
            .collect();

        Self {
            id,
            winning_numbers,
            own_numbers,
        }
    }

    fn own_winning_numbers(&self) -> impl Iterator<Item = i32> + '_ {
        self.winning_numbers
            .iter()
            .filter(|n| self.own_numbers.contains(n))
            .copied()
    }
}

struct Deck {
    original_cards: Vec<Card>,
    cards: Vec<Card>,
}

impl Deck {
    fn parse(puzzle: &str) -> Self {
        let cards: Vec<_> = puzzle.lines().map(Card::parse).collect();
        Self {
            original_cards: cards.clone(),
            cards,
        }
    }

    fn nextn(&self, id: i32, n: i32) -> impl Iterator<Item = &Card> + '_ {
        self.original_cards
            .iter()
            .skip_while(move |c| c.id != id)
            .skip(1)
            .take(n as usize)
    }

    fn put_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn len(&self) -> i32 {
        self.cards.len() as i32
    }

    fn at(&self, index: i32) -> &Card {
        &self.cards[index as usize]
    }
}

fn solve(puzzle: &str) -> i32 {
    let puzzle = normalize(puzzle);
    let mut deck = Deck::parse(&puzzle);
    let mut index = 0;

    while index < deck.len() {
        let card = deck.at(index);
        let matches = card.own_winning_numbers().count();
        let nextn = deck
            .nextn(card.id, matches as i32)
            .cloned()
            .collect::<Vec<_>>();
        for c in nextn {
            deck.put_card(c);
        }
        index += 1;
    }

    deck.len()
}

#[aocd(2023, 4)]
fn main() {
    let input = input!();
    let result = solve(&input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PUZZLE: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11    
";

    #[test]
    fn test_normalize() {
        let puzzle = "Card   9: 92 39 44 48 96 72 43 78 98 86 | 76 87  9 45 98 47 31 44 34 92 43 54 72 39 50 96 77 86 62 13 16  6 78 48  8
Card  10: 66 44 15 56 88 27 54 51  5 92 | 44 92 18 56 22 85 40 76 90 83  5 13 35 59 27 65 62 15 95 94 81 39 88 54  6";

        let expected = "9|92 39 44 48 96 72 43 78 98 86|76 87 9 45 98 47 31 44 34 92 43 54 72 39 50 96 77 86 62 13 16 6 78 48 8
10|66 44 15 56 88 27 54 51 5 92|44 92 18 56 22 85 40 76 90 83 5 13 35 59 27 65 62 15 95 94 81 39 88 54 6";

        assert_eq!(normalize(puzzle), expected);
    }

    #[test]
    fn test_solve_against_example() {
        assert_eq!(solve(EXAMPLE_PUZZLE), 30);
    }

    #[test]
    fn test_nextn() {
        let deck = Deck::parse(&normalize(EXAMPLE_PUZZLE));
        let nextn = deck.nextn(1, 2).collect::<Vec<_>>();
        assert_eq!(nextn.len(), 2);
        assert_eq!(nextn[0].id, 2);
        assert_eq!(nextn[1].id, 3);
    }
}
