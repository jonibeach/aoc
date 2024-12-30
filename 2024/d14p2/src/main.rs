use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

const INPUT: &'static str = include_str!("../../../input.txt");

#[derive(Debug)]
struct Robot {
    row: isize,
    col: isize,
    d_row: isize,
    d_col: isize,
}

fn print_robots<const W: usize, const H: usize>(robots: &Vec<Robot>) {
    let mut robot_counts = [['.'; W]; H];

    for r in robots {
        let count = &mut robot_counts[r.row as usize][r.col as usize];
        let new = match count {
            '.' => '1',
            '1'..'8' => (*count as u8 + 1) as char,
            _ => unimplemented!(),
        };

        *count = new;
    }

    for r in robot_counts {
        for c in r {
            print!("{c}")
        }
        print!("\r\n")
    }

    println!("\r\n");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut robots: Vec<Robot> = Vec::new();

    for l in INPUT.lines() {
        let (pos_str, v_str) = l.split_once(" ").unwrap();

        let pos_str = pos_str.trim().replace("p=", "");
        let (col_str, row_str) = pos_str.split_once(",").unwrap();
        let (col, row) = (col_str.parse().unwrap(), row_str.parse().unwrap());

        let v_str = v_str.trim().replace("v=", "");
        let (d_col_str, d_row_str) = v_str.split_once(",").unwrap();
        let (d_col, d_row) = (d_col_str.parse().unwrap(), d_row_str.parse().unwrap());

        robots.push(Robot {
            row,
            col,
            d_row,
            d_col,
        });
    }

    const W: usize = 101;
    const H: usize = 103;
    const W_I: isize = W as isize;
    const H_I: isize = H as isize;

    enable_raw_mode()?;

    let mut total_iters = 0;

    loop {
        if event::poll(std::time::Duration::from_millis(500))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.code == KeyCode::Char(' ') {
                    let iters = if total_iters == 0 { 9 } else { 101 };
                    for _ in 0..iters {
                        for r in robots.iter_mut() {
                            r.row += r.d_row;
                            r.col += r.d_col;

                            if r.row < 0 {
                                r.row = H_I + r.row
                            }

                            if r.row - H_I >= 0 {
                                r.row -= H_I
                            }

                            if r.col < 0 {
                                r.col = W_I + r.col
                            }

                            if r.col - W_I >= 0 {
                                r.col -= W_I
                            }
                        }
                        total_iters += 1;
                    }
                    print_robots::<W, H>(&robots);
                } else {
                    break;
                }
            }
        }
    }

    disable_raw_mode()?;

    let mut quadrants = [0; 4];

    let mid_col = (W_I + 1) / 2 - 1;
    let mid_row = (H_I + 1) / 2 - 1;

    for r in robots {
        if r.col == mid_col || r.row == mid_row {
            continue;
        }

        let quadrant_idx = match (r.col > mid_col, r.row > mid_row) {
            (false, false) => 0,
            (true, false) => 1,
            (false, true) => 2,
            (true, true) => 3,
        };

        quadrants[quadrant_idx] += 1;
    }

    let res: usize = quadrants.iter().fold(1, |res, curr| res * curr);

    println!("{res}");

    Ok(())
}
