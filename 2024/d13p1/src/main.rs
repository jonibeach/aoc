use std::collections::HashMap;

const INPUT: &'static str = include_str!("../../../input.txt");

struct Button {
    cost: isize,
    x: isize,
    y: isize,
}

impl Button {
    fn from_str(s: &str) -> Self {
        let (n, rest) = s.split_once(":").unwrap();

        let len = n.len();
        let cost = match &n[len - 1..len] {
            "A" => 3,
            "B" => 1,
            _ => unimplemented!(),
        };

        let (first, second) = rest.split_once(",").unwrap();
        let x = first.trim().replace("X+", "").parse().unwrap();
        let y = second.trim().replace("Y+", "").parse().unwrap();

        Self { cost, x, y }
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Prize {
    x: isize,
    y: isize,
}

impl Prize {
    fn from_str(s: &str) -> Self {
        let (_, rest) = s.split_once(":").unwrap();

        let (first, second) = rest.split_once(",").unwrap();
        let x: isize = first.trim().replace("X=", "").parse().unwrap();
        let y: isize = second.trim().replace("Y=", "").parse().unwrap();

        // the extra 1000... for P2
        let x = 10000000000000 + x;
        let y = 10000000000000 + y;

        Self { x, y }
    }
}

struct Machine {
    btn_a: Button,
    btn_b: Button,
    prize: Prize,
}

impl Machine {
    fn from_chunk(c: &[&str]) -> Self {
        let btn_a = Button::from_str(c[0]);
        let btn_b = Button::from_str(c[1]);
        let prize = Prize::from_str(c[2]);

        Self {
            btn_a,
            btn_b,
            prize,
        }
    }
}

fn find_cheapest(machine: &Machine) -> Option<isize> {
    let (a1, b1, a2, b2) = (
        machine.btn_a.x,
        machine.btn_a.y,
        machine.btn_b.x,
        machine.btn_b.y,
    );
    let (xt, yt) = (machine.prize.x, machine.prize.y);
    let a_x = (a1 * b2 * xt - a1 * a2 * yt) / (b2 * a1 - a2 * b1);

    let a_count = a_x / machine.btn_a.x;
    let b_count = (machine.prize.x - a_x) / machine.btn_b.x;

    if a_count * machine.btn_a.x + b_count * machine.btn_b.x == machine.prize.x
        && a_count * machine.btn_a.y + b_count * machine.btn_b.y == machine.prize.y
    {
        Some(a_count * 3 + b_count)
    } else {
        None
    }
}

fn main() {
    let machines: Vec<_> = INPUT
        .lines()
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>()
        .chunks_exact(3)
        .map(|c| Machine::from_chunk(c))
        .collect();

    let mut res = 0;

    for m in machines {
        if let Some(c) = find_cheapest(&m) {
            res += c;
        }
    }

    println!("{res}");
}
