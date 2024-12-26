const INPUT: &'static str = include_str!("../../../input.txt");

fn blink(stones: &mut Vec<usize>) {
    let mut i = 0;

    while let Some(s) = stones.get(i).copied() {
        if s == 0 {
            stones[i] = 1;
        } else {
            let digits = (s as f64).log10().floor() as usize + 1;
            if digits % 2 == 0 {
                let str = format!("{s}");
                let (a, b) = str.split_at(digits / 2);
                let (a, b) = (a.parse().unwrap(), b.parse().unwrap());
                stones[i] = a;
                stones.insert(i + 1, b);
                i += 1;
            } else {
                *&mut stones[i] *= 2024;
            }
        }
        i += 1;
    }
}

fn main() {
    let mut stones: Vec<usize> = INPUT.split(" ").filter_map(|v| v.parse().ok()).collect();

    for i in 0..75 {
        blink(&mut stones);
        println!("{i}");
        //println!("{stones:?}");
        //println!("{} blinks -> {} stones", i + 1, stones.len());
    }

    println!("{}", stones.len())
}
