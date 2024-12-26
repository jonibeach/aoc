use std::collections::HashSet;

const INPUT: &'static str = include_str!("../../../input.txt");

enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl Dir {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '<' => Some(Self::Left),
            '>' => Some(Self::Right),
            '^' => Some(Self::Up),
            'v' => Some(Self::Down),
            _ => None,
        }
    }
    // fn to_char(&self) -> char {
    //     match self {
    //         Self::Left => '<',
    //         Self::Right => '>',
    //         Self::Up => '^',
    //         Self::Down => 'v',
    //     }
    // }
    fn turn_right(&self) -> Self {
        match self {
            Self::Left => Self::Up,
            Self::Right => Self::Down,
            Self::Up => Self::Right,
            Self::Down => Self::Left,
        }
    }
    fn mv(&self, row: usize, col: usize) -> Option<(usize, usize)> {
        match self {
            Self::Left => Some((
                row,
                if let Some(s) = col.checked_sub(1) {
                    s
                } else {
                    return None;
                },
            )),
            Self::Right => Some((row, col + 1)),
            Self::Up => Some((
                if let Some(s) = row.checked_sub(1) {
                    s
                } else {
                    return None;
                },
                col,
            )),
            Self::Down => Some((row + 1, col)),
        }
    }
}

struct Guard {
    row: usize,
    col: usize,
    dir: Dir,
}

impl Guard {
    fn mv(&mut self, grid: &mut Vec<String>, visited: &mut HashSet<(usize, usize)>) -> Option<()> {
        visited.insert((self.row, self.col));

        let (next_row, next_col) = if let Some(n) = self.dir.mv(self.row, self.col) {
            n
        } else {
            return None;
        };

        if next_row >= grid[0].len() || next_col >= grid.len() {
            return None;
        }

        if grid[next_row].chars().nth(next_col).unwrap() == '#' {
            self.dir = self.dir.turn_right();
        } else {
            // grid[self.row].replace_range(self.col..self.col + 1, ".");
            self.row = next_row;
            self.col = next_col;
            // grid[self.row].replace_range(self.col..self.col + 1, &self.dir.to_char().to_string());
        }

        Some(())
    }
}

fn main() { 
    let mut grid: Vec<String> = INPUT.lines().map(|s| s.to_string()).collect();
    let mut visited = HashSet::new();
    let (row, col, ch) = grid
        .iter()
        .enumerate()
        .flat_map(|(r, cont)| cont.char_indices().map(move |(c, ch)| (r, c, ch)))
        .find(|(_, _, ch)| Dir::from_char(*ch).is_some())
        .unwrap();

    let dir = Dir::from_char(ch).unwrap();

    let mut guard = Guard { row, col, dir };

    // println!("{grid:#?}");
    while Some(()) == guard.mv(&mut grid, &mut visited) {
        // println!("{grid:#?}");
    }
    
    

    println!("{}", visited.len());
}
