use std::collections::BTreeMap;

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

fn map_grid(
    grid: &Vec<Vec<char>>,
    curr: (usize, usize),
    checked: &mut Vec<Vec<bool>>,
    // Stores the rows and width of each row.
    row_widths: &mut BTreeMap<usize, usize>,
    area: &mut usize,
) {
    if !checked[curr.0][curr.1] {
        *row_widths.entry(curr.0).or_insert(0) += 1;
        *area += 1;
        checked[curr.0][curr.1] = true;

        for (row, col) in neighbours(&grid, curr) {
            if grid[row][col] == grid[curr.0][curr.1] {
                map_grid(grid, (row, col), checked, row_widths, area);
            }
        }
    }
}

fn main() {
    let grid: Vec<Vec<char>> = INPUT.lines().map(|l| l.chars().collect()).collect();
    let mut checked: Vec<Vec<bool>> = grid
        .iter()
        .map(|l| l.iter().map(|_| false).collect())
        .collect();

    let mut areas = Vec::new();

    for (row, r) in grid.iter().enumerate() {
        for (col, _) in r.iter().enumerate() {
            let mut row_widths = BTreeMap::new();
            let mut area = 0;
            map_grid(&grid, (row, col), &mut checked, &mut row_widths, &mut area);
            if row_widths.len() > 0 {
                areas.push((area, row_widths));
            }
        }
    }

    let mut res = 0;

    for (total_area, rows) in areas {
        let mut prev_width = None;
        let mut sides = 0;
        for (_row, width) in rows {
            if let Some(prev) = prev_width {
                if prev != width {
                    sides += 2;
                }
            } else {
                sides += 4;
            }
            prev_width = Some(width)
        }
        res += total_area * sides
    }

    println!("{res}");
}
