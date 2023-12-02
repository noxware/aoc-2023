fn normalize(puzzle: &str) -> String {
    puzzle
        .trim()
        .to_lowercase()
        .replace(|c| c == ':' || c == ';', ",")
        .replace(", ", ",")
}

fn tokenize(puzzle: &str) -> impl Iterator<Item = impl Iterator<Item = (&str, &str)>> {
    puzzle.lines().map(|game| {
        game.split(',')
            .map(|s| s.split_once(' '))
            .map(|o| o.expect("no space character while tokenizing"))
    })
}

fn solve(puzzle: &str) -> u32 {
    let puzzle = normalize(puzzle);
    let puzzle = tokenize(&puzzle);

    puzzle.fold(0, |acc, mut game| {
        game.next().expect("can not read game tokens");

        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for (count, color) in game {
            let count: u32 = count.parse().expect("subset count is not a number");

            match color {
                "red" => {
                    if count > min_red {
                        min_red = count;
                    }
                }
                "green" => {
                    if count > min_green {
                        min_green = count;
                    }
                }
                "blue" => {
                    if count > min_blue {
                        min_blue = count;
                    }
                }
                _ => panic!("unexpected color"),
            }
        }

        let power = min_red * min_green * min_blue;

        power + acc
    })
}

fn main() {
    let input = std::fs::read_to_string("inputs/d2.txt").expect("Unable to read input");
    let result = solve(&input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    const PUZZLE: &str = "Game 1: 10 green, 5 blue; 1 red, 9 green, 10 blue
Game 2: 7 green, 5 red, 3 blue
";

    const EXAMPLE_PUZZLE: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    #[test]
    fn test_normalize() {
        let expected = "game 1,10 green,5 blue,1 red,9 green,10 blue\ngame 2,7 green,5 red,3 blue";
        assert_eq!(normalize(PUZZLE), expected);
    }

    #[test]
    fn test_tokenize() {
        let normalized = normalize(PUZZLE);
        let result = tokenize(&normalized)
            .map(|game| game.collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let expected = vec![
            vec![
                ("game", "1"),
                ("10", "green"),
                ("5", "blue"),
                ("1", "red"),
                ("9", "green"),
                ("10", "blue"),
            ],
            vec![("game", "2"), ("7", "green"), ("5", "red"), ("3", "blue")],
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_solve_against_example() {
        assert_eq!(solve(EXAMPLE_PUZZLE), 2286);
    }
}
