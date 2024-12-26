const INPUT: &'static str = include_str!("../../../input.txt");
const INPUT_STR: [char; 4] = ['X', 'M', 'A', 'S'];
const DELTAS: [(isize, isize); 8] = [
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
];

#[derive(Clone, Copy, Default, Debug)]
struct Coords(usize, usize);

impl Coords {
    fn apply_delta(&self, max: &Self, delta: (isize, isize)) -> Option<Self> {
        if let (Some(new_x), Some(new_y)) = (
            self.0.checked_add_signed(delta.0),
            self.1.checked_add_signed(delta.1),
        ) {
            if new_x >= max.0 || new_y >= max.1 {
                return None;
            }
            return Some(Self(new_x, new_y));
        }

        None
    }
}

fn search(
    grid: &Vec<String>,
    max: Coords,
    coords: Coords,
    curr: Option<((isize, isize), [Coords; 4])>,
    mut next_char_idx: usize,
) -> Vec<[Coords; 4]> {
    let mut res = Vec::new();

    let c = grid[coords.1].chars().nth(coords.0).unwrap();

    if c == INPUT_STR[next_char_idx] {
        let temp;

        let (iter, mut curr_find) = if let Some((dir, find)) = curr {
            temp = [dir];
            (temp.iter(), find)
        } else {
            (DELTAS.iter(), [Coords::default(); 4])
        };

        curr_find[next_char_idx] = coords;

        next_char_idx += 1;

        if next_char_idx <= 3 {
            for delta in iter {
                if let Some(new_coords) = coords.apply_delta(&max, *delta) {
                    res.extend_from_slice(&search(
                        grid,
                        max,
                        new_coords,
                        Some((*delta, curr_find)),
                        next_char_idx,
                    ));
                }
            }
        } else {
            res.push(curr.unwrap().1);
        };
    }

    res
}

fn main() {
    let grid: Vec<String> = INPUT.lines().map(|l| l.to_string()).collect();

    let max = Coords(grid[0].len(), grid.len());

    let mut res = 0;

    for (y, line) in grid.iter().enumerate() {
        for x in 0..line.len() {
            let s_res = search(&grid, max, Coords(x, y), None, 0);
            res += s_res.len();
        }
    }

    println!("{res}");
}
