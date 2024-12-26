const INPUT: &'static str = include_str!("../../../input.txt");

fn main() {
    let mut first: Vec<u64> = Vec::new();
    let mut second: Vec<u64> = Vec::new();

    for line in INPUT.lines() {
        if let Some((s1, rest)) = line.split_once(" ") {
            let s2 = rest.trim();
            if let (Ok(n1), Ok(n2)) = (s1.parse(), s2.parse()) {
                first.push(n1);
                second.push(n2);
            }
        }
    }

    first.sort();
    second.sort();

    let mut res = 0;
    let mut i = 0;
    while i < first.len() {
        res += first[i].abs_diff(second[i]);
        i += 1;
    }

    println!("{res}")
}
