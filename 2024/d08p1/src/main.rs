use std::collections::{HashMap, HashSet};

const INPUT: &'static str = include_str!("../../../input.txt");

fn delta_within_bounds(coords: (i8, i8), delta: (i8, i8), max: (usize, usize)) -> (bool, (i8, i8)) {
    let coords = (coords.0 + delta.0, coords.1 + delta.1);
    let within_bounds =
        coords.0 < max.0 as i8 && coords.1 < max.1 as i8 && coords.0 >= 0 && coords.1 >= 0;

    (within_bounds, coords)
}

fn main() {
    let grid: Vec<String> = INPUT.lines().map(|l| l.to_string()).collect();
    let mut antennas: HashMap<char, Vec<(i8, i8)>> = HashMap::new();

    for (r, l) in grid.iter().enumerate() {
        for (c, ch) in l.char_indices() {
            if ch == '.' {
                continue;
            }

            antennas
                .entry(ch)
                .or_insert(vec![])
                .push((r as i8, c as i8));
        }
    }

    let max = (grid.len(), grid[0].len());

    let mut res = HashSet::new();
    for (_, antennas) in antennas {
        let mut a = 0;
        while let Some((row1, col1)) = antennas.get(a) {
            let mut b = a + 1;
            while let Some((row2, col2)) = antennas.get(b) {
                let delta = (row1 - row2, col1 - col2);
                if let (true, coords) = delta_within_bounds((*row1, *col1), delta, max) {
                    res.insert(coords);
                }
                if let (true, coords) =
                    delta_within_bounds((*row2, *col2), (-delta.0, -delta.1), max)
                {
                    res.insert(coords);
                }
                b += 1;
            }
            a += 1;
        }
    }

    // for (r, c) in &res {
    //     grid[*r as usize].replace_range(*c as usize..*c as usize + 1, "#");
    // }

    println!("{}", res.len());
}
