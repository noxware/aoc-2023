fn try_parse_digit(slice: &[char]) -> Option<u32> {
    if let Some(digit) = slice[0].to_digit(10) {
        return Some(digit);
    }

    match slice {
        ['o', 'n', 'e', ..] => Some(1),
        ['t', 'w', 'o', ..] => Some(2),
        ['t', 'h', 'r', 'e', 'e', ..] => Some(3),
        ['f', 'o', 'u', 'r', ..] => Some(4),
        ['f', 'i', 'v', 'e', ..] => Some(5),
        ['s', 'i', 'x', ..] => Some(6),
        ['s', 'e', 'v', 'e', 'n', ..] => Some(7),
        ['e', 'i', 'g', 'h', 't', ..] => Some(8),
        ['n', 'i', 'n', 'e', ..] => Some(9),
        _ => None,
    }
}

fn main() {
    let result: u32 = std::fs::read_to_string("inputs/d1.txt")
        .expect("Unable to read input")
        .lines()
        .map(|l| {
            let chars: Vec<_> = l.chars().collect();
            let mut first: Option<u32> = None;
            let mut last: Option<u32> = None;

            for i in 0..chars.len() {
                let slice = &chars[i..];
                if let Some(digit) = try_parse_digit(slice) {
                    if first.is_none() {
                        first = Some(digit);
                    }

                    last = Some(digit);
                }
            }

            return first.unwrap() * 10 + last.unwrap();
        })
        .sum();

    println!("{}", result);
}
