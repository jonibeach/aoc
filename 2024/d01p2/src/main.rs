use std::collections::HashMap;

const INPUT: &'static str = include_str!("../../../input.txt");

fn main() {
    let mut first: Vec<u64> = Vec::new();
    let mut second: Vec<u64> = Vec::new();
    let mut second_freq: HashMap<u64, u64> = HashMap::new();

    for line in INPUT.lines() {
        if let Some((s1, rest)) = line.split_once(" ") {
            let s2 = rest.trim();
            if let (Ok(n1), Ok(n2)) = (s1.parse(), s2.parse()) {
                first.push(n1);
                second.push(n2);
                *second_freq.entry(n2).or_insert(0) += 1;
            }
        }
    }

    let mut res = 0;
    let mut i = 0;

    while i < first.len() {
        let val = first[i];
        let second_freq = second_freq.get(&val).unwrap_or(&0);
        res += val * second_freq;
        i += 1;
    }

    println!("{res}")
}
