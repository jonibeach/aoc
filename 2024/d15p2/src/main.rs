use std::{
    collections::BTreeSet,
    fmt::{Debug, Write},
    thread,
    time::Duration,
};

const INPUT: &'static str = include_str!("../../../input.txt");

#[derive(PartialEq, Clone, Copy)]
enum Square {
    Wall,
    BoxLeft,
    BoxRight,
    Empty,
    Robot,
}

impl Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Wall => f.write_char('#'),
            Self::BoxLeft => f.write_char('['),
            Self::BoxRight => f.write_char(']'),
            Self::Empty => f.write_char('.'),
            Self::Robot => f.write_char('@'),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Up,
    Down,
    Right,
    Left,
}

impl Move {
    fn mv(&self, coords: (usize, usize)) -> (usize, usize) {
        let (row, col) = coords;
        match self {
            Self::Up => (row - 1, col),
            Self::Down => (row + 1, col),
            Self::Right => (row, col + 1),
            Self::Left => (row, col - 1),
        }
    }
}

fn find_boxes_recursive(
    grid: &Vec<Vec<Square>>,
    curr: &[(usize, usize)],
    mv: Move,
) -> Option<BTreeSet<(usize, usize)>> {
    let mut res = BTreeSet::new();
    res.extend(curr);
    match mv {
        Move::Left | Move::Right => {
            let robot_pos = curr[0];
            let mut curr_pos = mv.mv(robot_pos);

            while let Square::BoxLeft | Square::BoxRight = grid[curr_pos.0][curr_pos.1] {
                res.insert(curr_pos);
                curr_pos = mv.mv(curr_pos);
            }

            if grid[curr_pos.0][curr_pos.1] == Square::Wall {
                return None;
            }
        }
        Move::Up | Move::Down => {
            for c in curr {
                println!("CHECKING {mv:?} FROM {c:?}");
                let next = mv.mv(*c);
                match grid[next.0][next.1] {
                    Square::Wall => {
                        println!("FOUND WALL");
                        return None;
                    }
                    Square::Empty | Square::Robot => {}
                    s => {
                        let next_col = match s {
                            Square::BoxLeft => next.1 + 1,
                            Square::BoxRight => next.1 - 1,
                            _ => unreachable!(),
                        };
                        if let Some(b) = find_boxes_recursive(grid, &[next, (next.0, next_col)], mv)
                        {
                            res.extend(b);
                        } else {
                            return None;
                        }
                    }
                };
            }
        }
    }
    Some(res)
}

fn main() {
    let mut grid = Vec::new();
    let mut robot_pos = (0, 0);

    let mut input = INPUT.replace("#", "##");
    input = input.replace("O", "[]");
    input = input.replace(".", "..");
    input = input.replace("@", "@.");

    let mut lines = input.lines();

    while let Some(l) = lines.next() {
        if l.is_empty() {
            break;
        }
        let mut line = Vec::new();

        for c in l.chars().collect::<Vec<char>>().chunks_exact(2) {
            let [c1, c2]: [char; 2] = c.try_into().unwrap();
            let slice = match (c1, c2) {
                ('#', '#') => &[Square::Wall, Square::Wall],
                ('[', ']') => &[Square::BoxLeft, Square::BoxRight],
                ('.', '.') => &[Square::Empty, Square::Empty],
                ('@', '.') => {
                    robot_pos = (grid.len(), line.len());
                    &[Square::Robot, Square::Empty]
                }
                _ => continue,
            };

            line.extend_from_slice(slice);
        }

        grid.push(line);
    }

    for c in lines.collect::<String>().chars() {
        let mv = match c {
            '^' => Move::Up,
            'v' => Move::Down,
            '>' => Move::Right,
            '<' => Move::Left,
            _ => continue,
        };
        grid.iter().for_each(|r| println!("{r:?}"));

        let new_pos = mv.mv(robot_pos);

        let new_content = &grid[new_pos.0][new_pos.1];

        match new_content {
            Square::Empty => {
                grid[robot_pos.0][robot_pos.1] = Square::Empty;
                robot_pos = new_pos;
                grid[robot_pos.0][robot_pos.1] = Square::Robot
            }
            Square::Wall => {}
            Square::BoxLeft | Square::BoxRight => {
                if let Some(sqs_to_move) = find_boxes_recursive(&grid, &[robot_pos], mv) {
                    for (row, col) in sqs_to_move {
                        let sq = grid[row][col];
                        grid[row][col] = Square::Empty;
                        let (new_row, new_col) = mv.mv((row, col));
                        grid[new_row][new_col] = sq;
                        if sq == Square::Robot {
                            robot_pos = (new_row, new_col);
                        }
                    }
                }
            }
            _ => unreachable!(),
        };
    }

    let mut res = 0;

    for ((row, col), _) in grid
        .iter()
        .enumerate()
        .flat_map(|(r, row)| row.iter().enumerate().map(move |(c, sq)| ((r, c), sq)))
        .filter(|(_, sq)| matches!(sq, Square::BoxLeft))
    {
        res += 100 * row + col;
    }

    grid.iter().for_each(|r| println!("{r:?}"));
    println!("{res}");
}
