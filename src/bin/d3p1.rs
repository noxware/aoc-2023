use std::collections::HashMap;

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Copy, Clone, Debug)]
enum PointValue {
    Digit(char),
    Symbol(char),
}

impl PointValue {
    fn from_char(value: char) -> Result<Self, ()> {
        if value == '.' || value == ' ' {
            return Err(());
        }

        if value.is_ascii_digit() {
            return Ok(Self::Digit(value));
        }

        Ok(Self::Symbol(value))
    }
}

struct EngineBounds {
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
}

struct Engine {
    schematic: HashMap<Point, PointValue>,
}

impl Engine {
    fn parse(s: &str) -> Self {
        let mut schematic: HashMap<Point, PointValue> = HashMap::new();

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if let Ok(value) = PointValue::from_char(c) {
                    schematic.insert(Point::new(x as i32, y as i32), value);
                }
            }
        }

        Self { schematic }
    }

    fn get(&self, x: i32, y: i32) -> Option<PointValue> {
        self.schematic.get(&Point::new(x, y)).copied()
    }

    fn get_adjacent(&self, x: i32, y: i32) -> [Option<PointValue>; 8] {
        [
            self.get(x - 1, y - 1),
            self.get(x, y - 1),
            self.get(x + 1, y - 1),
            self.get(x - 1, y),
            self.get(x + 1, y),
            self.get(x - 1, y + 1),
            self.get(x, y + 1),
            self.get(x + 1, y + 1),
        ]
    }

    fn max_x(&self) -> Option<i32> {
        self.schematic.keys().map(|p| p.x).max()
    }

    fn max_y(&self) -> Option<i32> {
        self.schematic.keys().map(|p| p.y).max()
    }

    fn min_x(&self) -> Option<i32> {
        self.schematic.keys().map(|p| p.x).min()
    }

    fn min_y(&self) -> Option<i32> {
        self.schematic.keys().map(|p| p.y).min()
    }

    fn bounds(&self) -> Option<EngineBounds> {
        Some(EngineBounds {
            min_x: self.min_x()?,
            min_y: self.min_y()?,
            max_x: self.max_x()?,
            max_y: self.max_y()?,
        })
    }

    fn is_part_number_component(&self, x: i32, y: i32) -> bool {
        self.get_adjacent(x, y)
            .iter()
            .any(|adj| matches!(adj, Some(PointValue::Symbol(_))))
    }
}

struct PartsNumberBuilder {
    part_numbers: Vec<i32>,
    current_buffer: String,
    is_part_number: bool,
}

impl PartsNumberBuilder {
    fn new() -> Self {
        Self {
            part_numbers: Vec::new(),
            current_buffer: String::new(),
            is_part_number: false,
        }
    }

    fn consume_buffer(&mut self) {
        if self.is_part_number {
            let number: i32 = self.current_buffer.parse().expect("invalid part number");
            self.part_numbers.push(number);
        }

        self.current_buffer.clear();
        self.is_part_number = false;
    }

    fn push_digit(&mut self, value: char) {
        self.current_buffer.push(value);
    }

    fn flag_part_number(&mut self) {
        self.is_part_number = true;
    }

    fn build(self) -> Vec<i32> {
        self.part_numbers
    }
}

fn solve(puzzle: &str) -> i32 {
    let engine = Engine::parse(puzzle);

    let EngineBounds {
        min_x,
        min_y,
        max_x,
        max_y,
    } = engine.bounds().expect("empty engine");

    let mut builder = PartsNumberBuilder::new();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            match engine.get(x, y) {
                Some(PointValue::Digit(digit)) => {
                    builder.push_digit(digit);
                    if engine.is_part_number_component(x, y) {
                        builder.flag_part_number();
                    }
                }
                _ => {
                    builder.consume_buffer();
                }
            }
        }

        builder.consume_buffer();
    }

    builder.build().iter().sum()
}

fn main() {
    let input = std::fs::read_to_string("inputs/d3.txt").expect("Unable to read input");
    let result = solve(&input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PUZZLE: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..    
";

    #[test]
    fn test_solve_against_example() {
        assert_eq!(solve(EXAMPLE_PUZZLE), 4361);
    }
}
