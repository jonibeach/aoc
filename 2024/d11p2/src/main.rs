use std::collections::HashMap;

const INPUT: &'static str = include_str!("../../../input.txt");

fn blink(stones: &mut HashMap<usize, usize>) {
    let mut new = HashMap::new();
    for (k, v) in &*stones {
        if *k == 0 {
            *new.entry(1).or_insert(0) += *v;
        } else {
            let digits = (*k as f64).log10().floor() as usize + 1;
            if digits % 2 == 0 {
                let str = format!("{}", *k);
                let (a, b) = str.split_at(digits / 2);
                let (a, b) = (a.parse().unwrap(), b.parse().unwrap());
                *new.entry(a).or_insert(0) += *v;
                *new.entry(b).or_insert(0) += *v;
            } else {
                let n = *k * 2024;
                *new.entry(n).or_insert(0) += *v;
            }
        }
    }
    *stones = new;
}

fn main() {
    let mut stones: HashMap<usize, usize> = INPUT
        .split(" ")
        .filter_map(|v| v.parse().ok())
        .map(|val| (val, 1))
        .collect();

    for _ in 0..75 {
        blink(&mut stones);
        println!(
            "{} -> {stones:?}",
            stones.iter().fold(0, |res, (_, v)| res + v)
        );
        //println!("{stones:?}");
        //println!("{} blinks -> {} stones", i + 1, stones.len());
    }

    let res = stones.iter().fold(0, |res, (_, v)| res + v);

    println!("{}", res)
}
