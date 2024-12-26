use std::{collections::HashMap, str::Split};

type Range = (usize, usize);

#[derive(Debug)]
struct MappingRange {
    from: Range,
    to: Range,
}

impl MappingRange {
    fn from_str(input: &str) -> Option<Self> {
        if let Some((to_start, rest)) = input.split_once(" ") {
            if let Some((from_start, length)) = rest.split_once(" ") {
                match (
                    from_start.parse::<usize>(),
                    to_start.parse::<usize>(),
                    length.parse::<usize>(),
                ) {
                    (Ok(from_start), Ok(to_start), Ok(length)) => {
                        return Some(Self {
                            from: (from_start, from_start + length - 1),
                            to: (to_start, to_start + length - 1),
                        })
                    }
                    _ => {}
                }
            }
        }
        None
    }
    fn get_mapped_ranges(&self, input: Range) -> Vec<Range> {
        let (to_start, _) = self.to;
        let (from_start, from_end) = self.from;
        let (range_start, range_end) = input;
        let mut ranges = Vec::new();
        if range_end < from_start || from_end < range_start {
            // println!("PUSHING NO INT input={:?}, from={:?}", input, self.from);
            ranges.push(input)
        } else {
            let inter_start = from_start.max(range_start);
            let inter_end = from_end.min(range_end);
            let shift = to_start as isize - from_start as isize;
            let intersection = (
                inter_start.checked_add_signed(shift).unwrap_or(0),
                inter_end.checked_add_signed(shift).unwrap_or(0),
            );

            ranges.push(intersection);

            if range_start < from_start {
                ranges.push((range_start, inter_start - 1));
            }

            if range_end > from_end {
                ranges.push((inter_end + 1, range_end));
            }
        }
        ranges
    }
}

#[derive(Debug)]
struct AnyMap {
    ranges: Vec<MappingRange>,
    next: Box<Option<AnyMap>>,
}

impl AnyMap {
    fn get_lowest_last_mapping_value_with_ranges(
        &self,
        ranges_to_map: Vec<Range>,
    ) -> Option<usize> {
        let mut next_ranges_to_map: HashMap<Range, Vec<Range>> = HashMap::new();
        for range in ranges_to_map {
            for mapping_range in &self.ranges {
                if let Some(already_mapped) = next_ranges_to_map.get(&range) {
                    if let Some(first_map) = already_mapped.get(0) {
                        if first_map != &range {
                            continue;
                        }
                    }
                }
                next_ranges_to_map.insert(range, mapping_range.get_mapped_ranges(range));
            }
        }

        let next_ranges = next_ranges_to_map
            .values()
            .flatten()
            .map(|val| *val)
            .collect::<Vec<Range>>();
        if let Some(next) = &*self.next {
            return next.get_lowest_last_mapping_value_with_ranges(next_ranges);
        }
        next_ranges.iter().map(|(start, _)| *start).min()
    }

    fn from_lines(mut lines: Split<'_, &str>) -> Option<Self> {
        let mut next = lines.next();
        while let Some(n) = next {
            if !n.is_empty() {
                break;
            }
            next = lines.next();
        }
        if let Some(title_line) = next {
            if let Some((_, rest)) = title_line.split_once("-") {
                if let Some((_, rest)) = rest.split_once("-") {
                    if let Some(_) = rest.split_once(" map:") {
                        let mut ranges = Vec::new();

                        loop {
                            if let Some(next) = lines.next() {
                                if !next.is_empty() {
                                    if let Some(range) = MappingRange::from_str(next) {
                                        ranges.push(range);
                                        continue;
                                    }
                                }
                            }
                            break;
                        }
                        return Some(Self {
                            ranges,
                            next: Box::new(Self::from_lines(lines)),
                        });
                    }
                }
            }
        }
        None
    }
}

pub fn part1(input: String) -> usize {
    let mut lines = input.split("\n");
    let seed_ranges: Vec<Range> = lines
        .next()
        .expect("seeds line should exist")
        .split_once("seeds: ")
        .expect("seeds line should have 'seeds: '")
        .1
        .split(" ")
        .map(|val| {
            let start = val.parse::<usize>().expect("start should be valid usize");
            // let length = val.get(1).expect("chunk should have length").parse::<usize>().expect("length should be valid usize");
            (start, start)
        })
        .collect::<Vec<Range>>();

    if let Some(map) = AnyMap::from_lines(lines) {
        if let Some(lowest) = map.get_lowest_last_mapping_value_with_ranges(seed_ranges) {
            return lowest;
        }
    }
    0
}

pub fn part2(input: String) -> usize {
    let mut lines = input.split("\n");
    let seed_ranges: Vec<Range> = lines
        .next()
        .expect("seeds line should exist")
        .split_once("seeds: ")
        .expect("seeds line should have 'seeds: '")
        .1
        .split(" ")
        .collect::<Vec<&str>>()
        .chunks(2)
        .map(|val| {
            let start = val
                .get(0)
                .expect("chunk should have start")
                .parse::<usize>()
                .expect("start should be valid usize");
            let length = val
                .get(1)
                .expect("chunk should have length")
                .parse::<usize>()
                .expect("length should be valid usize");
            (start, start + length - 1)
        })
        .collect::<Vec<Range>>();

    if let Some(map) = AnyMap::from_lines(lines) {
        if let Some(lowest) = map.get_lowest_last_mapping_value_with_ranges(seed_ranges) {
            return lowest;
        }
    }
    0
}

fn main() {}
