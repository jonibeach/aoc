#[derive(Debug)]
struct Schematic {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

impl Schematic {
    fn from_string(input: String) -> Self {
        let mut schematic = Schematic {
            numbers: Vec::new(),
            symbols: Vec::new(),
        };
        for (y, line) in input.split("\n").enumerate() {
            let mut current_number: Option<Number> = None;
            for (x, char) in line.chars().enumerate() {
                if let Ok(number) = char.to_string().parse::<isize>() {
                    if let Some(ref mut current) = current_number {
                        current.value *= 10;
                        current.value += number;
                    } else {
                        current_number = Some(Number {
                            y: y as isize,
                            x_start: x as isize,
                            value: number,
                        })
                    }
                } else {
                    if let Some(current) = current_number {
                        schematic.numbers.push(current);
                        current_number = None;
                    }
                    match char {
                        '.' => {}
                        value => schematic.symbols.push(Symbol {
                            value,
                            x: x as isize,
                            y: y as isize,
                            adjacent_numbers: Vec::new(),
                        }),
                    }
                }
            }
            if let Some(current) = current_number {
                schematic.numbers.push(current);
            }
        }
        schematic
    }
    fn part_numbers_sum(&self) -> isize {
        let mut sum = 0;
        for number in &self.numbers {
            if number.is_adj_to_symbol(&self.symbols) {
                sum += number.value
            }
        }
        sum
    }
    fn fill_adjacent_numbers(&mut self) {
        for symbol in &mut self.symbols {
            match symbol.value {
                '*' => {
                    for number in &self.numbers {
                        if symbol.adjacent_numbers.len() < 2 {
                            let x_end = number.get_x_end();
                            if symbol.x >= number.x_start - 1
                                && symbol.x <= x_end + 1
                                && symbol.y >= number.y - 1
                                && symbol.y <= number.y + 1
                            {
                                symbol.adjacent_numbers.push(number.value)
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
    fn gear_ratio_sum(&self) -> isize {
        let mut sum = 0;
        for symbol in &self.symbols {
            if let Some(gear_ratio) = symbol.get_gear_ratio() {
                sum += gear_ratio
            }
        }
        sum
    }
}

#[derive(Debug)]
struct Symbol {
    x: isize,
    y: isize,
    value: char,
    adjacent_numbers: Vec<isize>,
}

impl Symbol {
    fn get_gear_ratio(&self) -> Option<isize> {
        match self.value {
            '*' => {
                let mut iter = self.adjacent_numbers.iter();
                if let Some(first) = iter.next() {
                    if let Some(second) = iter.next() {
                        return Some(first * second);
                    }
                }
                None
            }
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Number {
    y: isize,
    x_start: isize,
    value: isize,
}
impl Number {
    fn get_x_end(&self) -> isize {
        self.x_start + self.value.to_string().len() as isize - 1
    }
    fn is_adj_to_symbol(&self, symbols: &Vec<Symbol>) -> bool {
        let x_end = self.get_x_end();
        for symbol in symbols {
            if symbol.x >= self.x_start - 1
                && symbol.x <= x_end + 1
                && symbol.y >= self.y - 1
                && symbol.y <= self.y + 1
            {
                return true;
            }
        }
        false
    }
}

pub fn part1(input: String) -> isize {
    let schematic = Schematic::from_string(input);
    schematic.part_numbers_sum()
}

pub fn part2(input: String) -> isize {
    let mut schematic = Schematic::from_string(input);
    schematic.fill_adjacent_numbers();
    schematic.gear_ratio_sum()
}

#[cfg(test)]
mod tests {
    use super::Schematic;
    #[test]
    fn one() {
        let input = "123....333\n...*....-.\n..........\n123+......";
        assert_eq!(
            Schematic::from_string(String::from(input)).part_numbers_sum(),
            123 * 2 + 333
        );
    }
}

fn main() {}