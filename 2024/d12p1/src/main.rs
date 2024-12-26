use std::collections::HashMap;

const INPUT: &'static str = include_str!("../../../input.txt");

fn neighbours(grid: &Vec<Vec<char>>, curr: (usize, usize)) -> Vec<(usize, usize)> {
    let mut res = Vec::new();

    if curr.0 > 0 {
        res.push((curr.0 - 1, curr.1));
    }

    if curr.0 < grid.len() - 1 {
        res.push((curr.0 + 1, curr.1))
    }

    if curr.1 > 0 {
        res.push((curr.0, curr.1 - 1))
    }

    if curr.1 < grid[0].len() - 1 {
        res.push((curr.0, curr.1 + 1))
    }

    res
}

fn get_grid_area(
    grid: &Vec<Vec<char>>,
    curr: (usize, usize),
    checked: &mut Vec<Vec<bool>>,
) -> Option<Vec<((usize, usize), usize)>> {
    if !checked[curr.0][curr.1] {
        let mut res = Vec::new();
        checked[curr.0][curr.1] = true;

        let mut neighbour_count = 0;

        for (row, col) in neighbours(&grid, curr) {
            if grid[row][col] == grid[curr.0][curr.1] {
                neighbour_count += 1;
                res.extend_from_slice(&get_grid_area(grid, (row, col), checked).unwrap_or(vec![]))
            }
        }

        res.push((curr, neighbour_count));
        Some(res)
    } else {
        None
    }
}

fn main() {
    let grid: Vec<Vec<char>> = INPUT.lines().map(|l| l.chars().collect()).collect();
    let mut checked: Vec<Vec<bool>> = grid
        .iter()
        .map(|l| l.iter().map(|_| false).collect())
        .collect();

    let mut areas = HashMap::new();

    for (row, r) in grid.iter().enumerate() {
        for (col, ch) in r.iter().enumerate() {
            if let Some(a) = get_grid_area(&grid, (row, col), &mut checked) {
                areas.entry(ch).or_insert(vec![]).push(a)
            }
        }
    }

    let mut res = 0;

    for (_, areas) in areas {
        for area in areas {
            let area_sqs = area.len();
            let mut area_perim = 0;
            for (_, neighbour_count) in area {
                area_perim += match neighbour_count {
                    0 => 4,
                    1 => 3,
                    2 => 2,
                    3 => 1,
                    4 => 0,
                    _ => unreachable!(),
                }
            }
            res += area_sqs * area_perim
        }
    }

    println!("{res}");
}
