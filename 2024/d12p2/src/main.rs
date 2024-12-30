use std::collections::BTreeSet;

const INPUT: &'static str = include_str!("../../../input.txt");

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Edge {
    Up,
    Down,
    Right,
    Left,
}

fn neighbours(grid: &Vec<Vec<char>>, curr: (usize, usize)) -> Vec<(Edge, Option<(usize, usize)>)> {
    let up_neigh = if curr.0 > 0 {
        Some((curr.0 - 1, curr.1))
    } else {
        None
    };

    let down_neigh = if curr.0 < grid.len() - 1 {
        Some((curr.0 + 1, curr.1))
    } else {
        None
    };

    let left_neigh = if curr.1 > 0 {
        Some((curr.0, curr.1 - 1))
    } else {
        None
    };

    let right_neigh = if curr.1 < grid[0].len() - 1 {
        Some((curr.0, curr.1 + 1))
    } else {
        None
    };

    vec![
        (Edge::Up, up_neigh),
        (Edge::Down, down_neigh),
        (Edge::Left, left_neigh),
        (Edge::Right, right_neigh),
    ]
}

fn map_grid(
    grid: &Vec<Vec<char>>,
    curr: (usize, usize),
    checked: &mut Vec<Vec<bool>>,
    edges: &mut BTreeSet<((usize, usize), Edge)>,
    area: &mut usize,
) {
    if !checked[curr.0][curr.1] {
        *area += 1;
        checked[curr.0][curr.1] = true;

        for (edge, neigh) in neighbours(&grid, curr) {
            if let Some((row, col)) = neigh {
                if grid[row][col] == grid[curr.0][curr.1] {
                    map_grid(grid, (row, col), checked, edges, area);
                    continue;
                }
            }

            edges.insert((curr, edge));
        }
    }
}

fn main() {
    let grid: Vec<Vec<char>> = INPUT.lines().map(|l| l.chars().collect()).collect();
    let mut checked: Vec<Vec<bool>> = grid
        .iter()
        .map(|l| l.iter().map(|_| false).collect())
        .collect();
    let mut all_edges = Vec::new();

    for (row, r) in grid.iter().enumerate() {
        for (col, _) in r.iter().enumerate() {
            let mut area = 0;
            let mut edges = BTreeSet::new();
            map_grid(&grid, (row, col), &mut checked, &mut edges, &mut area);
            if edges.len() > 0 {
                all_edges.push((area, edges));
            }
        }
    }

    let mut res = 0;

    let (width, height) = (grid[0].len(), grid.len());

    for (area, edges) in all_edges {
        let mut edge_count = 0;
        let mut handled = BTreeSet::new();

        for edge in &edges {
            if !handled.contains(edge) {
                let (coords, dir) = edge;
                let (mut row, mut col) = coords;

                let incr = |row: &mut usize, col: &mut usize| {
                    match dir {
                        Edge::Down | Edge::Up => *col += 1,
                        Edge::Left | Edge::Right => *row += 1,
                    };
                    *col >= width || *row >= height
                };

                if !incr(&mut row, &mut col) {
                    while let Some(e) = edges.get(&((row, col), *dir)) {
                        handled.insert(e);
                        if incr(&mut row, &mut col) {
                            break;
                        }
                    }
                }

                edge_count += 1;
            }
        }
        res += area * edge_count;
    }

    println!("{res}");
}
