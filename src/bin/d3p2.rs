use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    const ZERO: Self = Self { x: 0, y: 0 };

    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Component {
    Number(i32),
    Symbol(char),
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Gear {
    ratio: i32,
}

impl Gear {
    fn new(ratio: i32) -> Self {
        Self { ratio }
    }
}

struct Engine {
    components: HashMap<Point, Component>,
    component_map: HashMap<Point, Point>,
}

impl Engine {
    fn new() -> Self {
        Self {
            components: HashMap::new(),
            component_map: HashMap::new(),
        }
    }

    fn resolve_position(&self, x: i32, y: i32) -> Point {
        let position = Point::new(x, y);
        *self.component_map.get(&position).unwrap_or(&position)
    }

    fn symbols(&self) -> HashMap<Point, char> {
        let mut symbols = HashMap::new();

        for (position, component) in &self.components {
            if let Component::Symbol(symbol) = component {
                symbols.insert(*position, *symbol);
            }
        }

        symbols
    }

    fn get_adjacent_components(&self, x: i32, y: i32) -> HashMap<Point, Component> {
        let mut deduper = HashMap::new();

        let deltas = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            // (0, 0),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        for (dx, dy) in deltas {
            let position = self.resolve_position(x + dx, y + dy);
            let component = self.components.get(&position).copied();

            if let Some(component) = component {
                deduper.insert(position, component);
            }
        }

        deduper
    }

    fn get_gears(&self) -> HashMap<Point, Gear> {
        let mut gears = HashMap::new();
        let symbols = self.symbols();

        for (position, symbol) in symbols {
            if symbol == '*' {
                let adjacents = self.get_adjacent_components(position.x, position.y);

                let numbers: Vec<_> = adjacents
                    .values()
                    .filter_map(|c| match c {
                        Component::Number(number) => Some(number),
                        _ => None,
                    })
                    .collect();

                if numbers.len() == 2 {
                    let ratio = numbers[0] * numbers[1];
                    gears.insert(position, Gear::new(ratio));
                }
            }
        }

        gears
    }
}

struct EngineBuilder {
    engine: Engine,
    digit_buffer: String,
    number_begin: Point,
}

impl EngineBuilder {
    fn new() -> Self {
        Self {
            engine: Engine::new(),
            digit_buffer: String::new(),
            number_begin: Point::ZERO,
        }
    }

    fn push_digit(&mut self, x: i32, y: i32, digit: char) {
        let position = Point::new(x, y);

        if self.digit_buffer.is_empty() {
            self.number_begin = position;
        }

        self.engine
            .component_map
            .insert(position, self.number_begin);

        self.digit_buffer.push(digit);
    }

    fn consume_digit_buffer(&mut self) {
        if self.digit_buffer.is_empty() {
            return;
        }

        let number: i32 = self
            .digit_buffer
            .parse()
            .expect("invalid number when consuming digit buffer");

        self.engine
            .components
            .insert(self.number_begin, Component::Number(number));

        self.digit_buffer.clear();
    }

    fn push_symbol(&mut self, x: i32, y: i32, symbol: char) {
        let position = Point::new(x, y);

        self.engine
            .components
            .insert(position, Component::Symbol(symbol));
        self.engine.component_map.insert(position, position);

        self.consume_digit_buffer();
    }

    fn push_void(&mut self) {
        self.consume_digit_buffer();
    }

    fn push(&mut self, x: i32, y: i32, c: char) {
        if c.is_ascii_digit() {
            self.push_digit(x, y, c);
        } else if c == '.' || c == ' ' {
            self.push_void();
        } else {
            self.push_symbol(x, y, c);
        }
    }

    fn build(self) -> Engine {
        self.engine
    }
}

fn solve(puzzle: &str) -> i32 {
    let mut builder = EngineBuilder::new();

    for (y, line) in puzzle.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            builder.push(x as i32, y as i32, c);
        }
        builder.push_void();
    }

    builder.build().get_gears().values().map(|g| g.ratio).sum()
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
        assert_eq!(solve(EXAMPLE_PUZZLE), 467835);
    }
}
