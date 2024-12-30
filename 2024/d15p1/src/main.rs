use std::fmt::{Debug, Write};

const INPUT: &'static str = include_str!("../../../input.txt");

#[derive(PartialEq, Clone, Copy)]
enum Square {
    Wall,
    Box,
    Empty,
    Robot,
}

impl Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Wall => f.write_char('#'),
            Self::Box => f.write_char('O'),
            Self::Empty => f.write_char('.'),
            Self::Robot => f.write_char('@'),
        }
    }
}

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

fn main() {
    let mut grid = Vec::new();
    let mut robot_pos = (0, 0);

    let mut lines = INPUT.lines();

    while let Some(l) = lines.next() {
        if l.is_empty() {
            break;
        }
        let mut line = Vec::new();

        for c in l.chars() {
            let sq = match c {
                '#' => Square::Wall,
                'O' => Square::Box,
                '.' => Square::Empty,
                '@' => {
                    robot_pos = (grid.len(), line.len());
                    Square::Robot
                }
                _ => continue,
            };

            line.push(sq);
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

        let new_pos = mv.mv(robot_pos);

        let new_content = &grid[new_pos.0][new_pos.1];

        match new_content {
            Square::Empty => {
                grid[robot_pos.0][robot_pos.1] = Square::Empty;
                robot_pos = new_pos;
                grid[robot_pos.0][robot_pos.1] = Square::Robot
            }
            Square::Wall => {}
            Square::Box => {
                let mut next_sq = mv.mv(new_pos);
                let mut next_sq_content = grid[next_sq.0][next_sq.1];

                while next_sq_content == Square::Box {
                    next_sq = mv.mv(next_sq);
                    next_sq_content = grid[next_sq.0][next_sq.1];
                }

                match next_sq_content {
                    Square::Empty => {
                        grid[robot_pos.0][robot_pos.1] = Square::Empty;
                        robot_pos = new_pos;
                        grid[robot_pos.0][robot_pos.1] = Square::Robot;
                        grid[next_sq.0][next_sq.1] = Square::Box;
                    }
                    Square::Wall => {}
                    _ => unreachable!(),
                }
            }
            _ => {
                grid.iter().for_each(|r| println!("{r:?}"));
                unreachable!();
            }
        };
    }

    let mut res = 0;

    for ((row, col), _) in grid
        .iter()
        .enumerate()
        .flat_map(|(r, row)| row.iter().enumerate().map(move |(c, sq)| ((r, c), sq)))
        .filter(|(_, sq)| matches!(sq, Square::Box))
    {
        res += 100 * row + col;
    }

    println!("{res}");
    grid.iter().for_each(|r| println!("{r:?}"));
}
