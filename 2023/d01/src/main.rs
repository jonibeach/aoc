fn concat_digits(a: usize, b: usize) -> usize {
    let mut temp = b;
    let mut digits = 1;

    while temp > 0 {
        temp /= 10;
        digits *= 10;
    }

    a * digits + b
}

#[derive(Clone, Copy, Debug)]
enum DigitLiteral {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

const ALL_DIGIT_LITERALS: [DigitLiteral; 9] = [
    DigitLiteral::One,
    DigitLiteral::Two,
    DigitLiteral::Three,
    DigitLiteral::Four,
    DigitLiteral::Five,
    DigitLiteral::Six,
    DigitLiteral::Seven,
    DigitLiteral::Eight,
    DigitLiteral::Nine,
];

impl DigitLiteral {
    fn to_str(&self) -> &'static str {
        let out = match self {
            Self::One => "one",
            Self::Two => "two",
            Self::Three => "three",
            Self::Four => "four",
            Self::Five => "five",
            Self::Six => "six",
            Self::Seven => "seven",
            Self::Eight => "eight",
            Self::Nine => "nine",
        };
        out
    }
    fn to_usize(&self) -> usize {
        let out = match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::Six => 6,
            Self::Seven => 7,
            Self::Eight => 8,
            Self::Nine => 9,
        };
        out
    }
    fn get_first(str: &str) -> Option<(usize, DigitLiteral)> {
        let mut lowest: Option<(usize, DigitLiteral)> = None;
        for digit in ALL_DIGIT_LITERALS {
            let match_indexes: Vec<_> = str
                .match_indices(digit.to_str())
                .map(|digit| digit.0)
                .collect();
            for index in match_indexes {
                if let Some((current_lowest, _)) = lowest {
                    if index < current_lowest {
                        lowest = Some((index, digit))
                    }
                } else {
                    lowest = Some((index, digit))
                }
            }
        }
        lowest
    }
    fn get_last(str: &str) -> Option<(usize, DigitLiteral)> {
        let mut highest: Option<(usize, DigitLiteral)> = None;
        for digit in ALL_DIGIT_LITERALS {
            let match_indexes: Vec<_> = str
                .match_indices(digit.to_str())
                .map(|digit| digit.0)
                .collect();
            for index in match_indexes {
                if let Some((current_lowest, _)) = highest {
                    if index > current_lowest {
                        highest = Some((index, digit))
                    }
                } else {
                    highest = Some((index, digit))
                }
            }
        }
        highest
    }
}

#[derive(Debug)]
enum Digit {
    Digit((usize, usize)),
    DigitLiteral((usize, DigitLiteral)),
}

pub fn part1(input: String) -> usize {
    let calibration_values = input.split("\n");
    let mut total: usize = 0;
    for calibration_value in calibration_values {
        let mut first: Option<usize> = None;
        let mut last: Option<usize> = None;

        for char in calibration_value.chars() {
            if let Ok(parsed) = char.to_string().parse::<usize>() {
                first = Some(parsed);
                break;
            };
        }
        for char in calibration_value.chars().rev() {
            if let Ok(parsed) = char.to_string().parse::<usize>() {
                last = Some(parsed);
                break;
            };
        }

        if let Some(first) = first {
            if let Some(last) = last {
                total += concat_digits(first, last);
            }
        }
    }
    total
}

pub fn part2(input: String) -> usize {
    let calibration_values = input.split("\n");
    let mut total: usize = 0;
    for calibration_value in calibration_values {
        let mut first_digit: Option<(usize, usize)> = None;
        let mut last_digit: Option<(usize, usize)> = None;
        let first_literal = DigitLiteral::get_first(calibration_value);
        let last_literal = DigitLiteral::get_last(calibration_value);

        for (idx, char) in calibration_value.chars().enumerate() {
            if let Ok(parsed) = char.to_string().parse::<usize>() {
                first_digit = Some((idx, parsed));
                break;
            };
        }
        let len = calibration_value.len();
        for (idx, char) in calibration_value.chars().rev().enumerate() {
            if let Ok(parsed) = char.to_string().parse::<usize>() {
                last_digit = Some(((len - 1) - idx, parsed));
                break;
            };
        }

        let mut first: Option<Digit> = None;
        let mut last: Option<Digit> = None;

        if let Some((digit_idx, digit)) = first_digit {
            if let Some((literal_idx, _)) = first_literal {
                if digit_idx < literal_idx {
                    first = Some(Digit::Digit((digit_idx, digit)));
                }
            } else {
                first = Some(Digit::Digit((digit_idx, digit)));
            }
        }

        if let Some((literal_idx, literal)) = first_literal {
            if let Some((digit_idx, _)) = first_digit {
                if literal_idx < digit_idx {
                    first = Some(Digit::DigitLiteral((literal_idx, literal)));
                }
            } else {
                first = Some(Digit::DigitLiteral((literal_idx, literal)));
            }
        }

        if let Some((digit_idx, digit)) = last_digit {
            if let Some((literal_idx, _)) = last_literal {
                if digit_idx > literal_idx {
                    last = Some(Digit::Digit((digit_idx, digit)));
                }
            } else {
                last = Some(Digit::Digit((digit_idx, digit)));
            }
        }

        if let Some((literal_idx, literal)) = last_literal {
            if let Some((digit_idx, _)) = last_digit {
                if literal_idx > digit_idx {
                    last = Some(Digit::DigitLiteral((literal_idx, literal)));
                }
            } else {
                last = Some(Digit::DigitLiteral((literal_idx, literal)));
            }
        }

        if let Some(first) = first {
            if let Some(last) = last {
                let first_value = match first {
                    Digit::DigitLiteral((_, literal)) => literal.to_usize(),
                    Digit::Digit((_, digit)) => digit,
                };
                let last_value = match last {
                    Digit::DigitLiteral((_, literal)) => literal.to_usize(),
                    Digit::Digit((_, digit)) => digit,
                };
                total += concat_digits(first_value, last_value);
            }
        }
    }
    total
}

fn main() {}
