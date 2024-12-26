const INPUT: &'static str = include_str!("../../../input.txt");

enum SearchDir {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

impl SearchDir {
    const SEARCH_THESE: [Self; 2] = [Self::TopLeft, Self::TopRight];

    fn opposite(&self) -> Self {
        match self {
            Self::TopLeft => Self::BottomRight,
            Self::TopRight => Self::BottomLeft,
            Self::BottomRight => Self::TopLeft,
            Self::BottomLeft => Self::TopRight,
        }
    }

    fn delta(&self) -> (isize, isize) {
        match self {
            Self::TopLeft => (-1, -1),
            Self::TopRight => (1, -1),
            Self::BottomRight => (1, 1),
            Self::BottomLeft => (-1, 1),
        }
    }
}

#[derive(PartialEq)]
enum SearchChar {
    M,
    S,
}

impl SearchChar {
    fn other(&self) -> Self {
        match self {
            Self::M => Self::S,
            Self::S => Self::M,
        }
    }

    fn from_char(c: char) -> Option<Self> {
        match c {
            'M' => Some(Self::M),
            'S' => Some(Self::S),
            _ => None,
        }
    }
}

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

fn search(grid: &Vec<String>, max: Coords, coords: Coords) -> usize {
    let mut res = 0;

    let c = |grid: &Vec<String>, coords: Coords| grid[coords.1].chars().nth(coords.0).unwrap();

    if c(grid, coords) == 'A' {
        let mut count = 0;
        for dir in SearchDir::SEARCH_THESE {
            if let Some(new_coords) = coords.apply_delta(&max, dir.delta()) {
                if let Some(first_c) = SearchChar::from_char(c(grid, new_coords)) {
                    if let Some(new_coords) = coords.apply_delta(&max, dir.opposite().delta()) {
                        if let Some(other_c) = SearchChar::from_char(c(grid, new_coords)) {
                            if other_c.other() == first_c {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
        if count == 2 {
            res += 1;
        }
    }

    res
}

fn main() {
    let grid: Vec<String> = INPUT.lines().map(|l| l.to_string()).collect();

    let max = Coords(grid[0].len(), grid.len());

    let mut res = 0;

    for (y, line) in grid.iter().enumerate() {
        for x in 0..line.len() {
            res += search(&grid, max, Coords(x, y));
        }
    }

    println!("{res}");
}
