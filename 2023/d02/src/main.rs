#[derive(Debug)]
struct Game {
    id: usize,
    rounds: Vec<Round>,
}

#[derive(Debug)]
struct Round {
    red: usize,
    green: usize,
    blue: usize,
}

impl Round {
    fn is_valid(&self) -> bool {
        if self.red > 12 {
            return false;
        }
        if self.green > 13 {
            return false;
        }
        if self.blue > 14 {
            return false;
        }
        true
    }
}

impl Game {
    fn is_valid(&self) -> bool {
        for round in &self.rounds {
            if !round.is_valid() {
                return false;
            }
        }
        true
    }
    fn from_str(input: &str) -> Option<Self> {
        if let Some((info, rounds_str)) = input.split_once(":") {
            if let Some((_, id)) = info.split_once(" ") {
                if let Ok(id) = id.parse::<usize>() {
                    let mut rounds = Vec::new();
                    for round_str in rounds_str.split(";") {
                        let mut round = Round {
                            red: 0,
                            green: 0,
                            blue: 0,
                        };
                        for round_color in
                            round_str.split(",").map(|round_color| round_color.trim())
                        {
                            if let Some((count, color)) = round_color.split_once(" ") {
                                if let Ok(count) = count.parse::<usize>() {
                                    match color {
                                        "red" => round.red = count,
                                        "green" => round.green = count,
                                        "blue" => round.blue = count,
                                        _ => {}
                                    }
                                }
                            }
                        }
                        rounds.push(round)
                    }
                    return Some(Self { id, rounds });
                }
            }
        }
        None
    }
    fn min_number_of_cubes(&self) -> Round {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for round in &self.rounds {
            if round.red > red {
                red = round.red
            }
            if round.green > green {
                green = round.green
            }
            if round.blue > blue {
                blue = round.blue
            }
        }
        Round { red, green, blue }
    }
}

pub fn part1(input: String) -> usize {
    let games = input.split("\n");
    let mut result: usize = 0;
    for game_str in games {
        let game = Game::from_str(game_str);
        if let Some(game) = game {
            if game.is_valid() {
                result += game.id;
            }
        }
    }
    result
}

pub fn part2(input: String) -> usize {
    let games = input.split("\n");
    let mut result: usize = 0;
    for game_str in games {
        let game = Game::from_str(game_str);
        if let Some(game) = game {
            let min_cubes = game.min_number_of_cubes();
            result += min_cubes.red * min_cubes.green * min_cubes.blue
        }
    }
    result
}

fn main() {}