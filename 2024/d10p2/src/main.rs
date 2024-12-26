const INPUT: &'static str = include_str!("../../../input.txt");

fn get_squares_around(t: &Vec<Vec<u8>>, curr: (usize, usize)) -> Vec<(usize, usize)> {
    let mut res = Vec::new();

    if curr.0 > 0 {
        res.push((curr.0 - 1, curr.1));
    }

    if curr.0 < t.len() - 1 {
        res.push((curr.0 + 1, curr.1))
    }

    if curr.1 > 0 {
        res.push((curr.0, curr.1 - 1))
    }

    if curr.1 < t[0].len() - 1 {
        res.push((curr.0, curr.1 + 1))
    }

    res
}

fn search(t: &Vec<Vec<u8>>, curr: (usize, usize)) -> usize {
    let mut diff_routes = 0;
    let curr_height = t[curr.0][curr.1];

    for sq in get_squares_around(t, curr)
        .iter()
        .filter(|(row, col)| t[*row][*col] == curr_height + 1)
    {
        let neighbour_height = t[sq.0][sq.1];
        if neighbour_height == 9 {
            diff_routes += 1;
        } else {
            diff_routes += search(t, *sq);
        }
    }
    diff_routes
}

fn main() {
    let mut topography = Vec::new();

    for l in INPUT.lines() {
        let l = Vec::from_iter(l.chars().map(|c| c as u8 - '0' as u8));
        topography.push(l);
    }

    let mut res = 0;
    for (r, row) in topography.iter().enumerate() {
        for (c, height) in row.iter().enumerate() {
            if height == &0 {
                let diff_routes = search(&topography, (r, c));
                res += diff_routes
            }
        }
    }

    println!("{res}");
}
