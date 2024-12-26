use std::collections::HashSet;

#[derive(Debug)]
struct Race {
    time_ms: usize,
    distance_mm: usize,
}

impl Race {
    const ACC: usize = 1;
    fn get_no_of_solutions(&self) -> usize {
        let mut sols = HashSet::new();
        for t1 in 0..=self.time_ms {
            let t2 = self.time_ms - t1;
            if t1 * Self::ACC * t2 > self.distance_mm {
                if sols.get(&(t1, t2)).is_none() {
                    sols.insert((t1, t2));
                }
            }
        }
        println!("TOTAL FOR GAME {:?} {}", self, sols.len());
        sols.len()
    }
    fn from_string_p1(input: String) -> Option<Vec<Self>> {
        let mut lines = input.split("\n");
        let line_1 = lines.next().expect("input should have 1st line");
        let line_2 = lines.next().expect("input should have 2nd line");
        if let Some((_, times_str)) = line_1.split_once("Time: ") {
            if let Some((_, distances_str)) = line_2.split_once("Distance: ") {
                let mut races = Vec::new();
                println!("{}", times_str);
                let mut times_iter = times_str
                    .trim()
                    .split(" ")
                    .filter(|val| !val.is_empty())
                    .map(|val| {
                        val.parse::<usize>()
                            .expect("all vals in times_str should be valid usizes")
                    });
                let mut distances_iter = distances_str
                    .trim()
                    .split(" ")
                    .filter(|val| !val.is_empty())
                    .map(|val| {
                        val.parse::<usize>()
                            .expect("all vals in distances_str should be valid usizes")
                    });

                loop {
                    if let Some(time) = times_iter.next() {
                        if let Some(distance) = distances_iter.next() {
                            races.push(Self {
                                time_ms: time,
                                distance_mm: distance,
                            });
                            continue;
                        }
                    }
                    break;
                }
                return Some(races);
            }
        }
        None
    }
    fn from_string_p2(input: String) -> Option<Self> {
        let mut lines = input.split("\n");
        let line_1 = lines.next().expect("input should have 1st line");
        let line_2 = lines.next().expect("input should have 2nd line");
        if let Some((_, times_str)) = line_1.split_once("Time: ") {
            if let Some((_, distances_str)) = line_2.split_once("Distance: ") {
                println!("{}", times_str);
                let time_ms = times_str
                    .trim()
                    .split(" ")
                    .filter(|val| !val.is_empty())
                    .map(|val| {
                        val.parse::<usize>()
                            .expect("all vals in times_str should be valid usizes")
                    })
                    .fold(0, |mut acc, val| {
                        acc *= 10usize.pow(val.to_string().len() as u32);
                        acc += val;
                        acc
                    });
                let distance_mm = distances_str
                    .trim()
                    .split(" ")
                    .filter(|val| !val.is_empty())
                    .map(|val| {
                        val.parse::<usize>()
                            .expect("all vals in distances_str should be valid usizes")
                    })
                    .fold(0, |mut acc, val| {
                        acc *= 10usize.pow(val.to_string().len() as u32);
                        acc += val;
                        acc
                    });
                return Some(Self {
                    time_ms,
                    distance_mm,
                });
            }
        }
        None
    }
}

pub fn part1(input: String) -> usize {
    let mut total = 1;
    if let Some(races) = Race::from_string_p1(input) {
        for race in races {
            total *= race.get_no_of_solutions()
        }
    }
    total
}

pub fn part2(input: String) -> usize {
    let mut total = 1;
    if let Some(race) = Race::from_string_p2(input) {
        total = race.get_no_of_solutions();
    }
    total
}

fn main() {}