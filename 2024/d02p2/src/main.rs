const INPUT: &'static str = include_str!("../../../input.txt");

fn check_report(report: &Vec<u64>) -> bool {
    let increasing = report[0] < report[1];
    let mut prev = &report[0];

    for level in &report[1..] {
        if (prev < level) != increasing {
            return false;
        }

        match level.abs_diff(*prev) {
            1..=3 => {}
            _ => return false,
        }
        prev = level;
    }

    true
}

fn main() {
    let mut res = 0;

    'outer: for mut report_str in INPUT.lines() {
        let mut report = Vec::new();

        while let Some((num, rest)) = report_str.split_once(" ") {
            if let Ok(num) = num.parse::<u64>() {
                report.push(num);
            }
            report_str = rest;
        }

        if let Ok(num) = report_str.parse::<u64>() {
            report.push(num);
        }

        // Bruteforce af lol
        for i in 0..report.len() {
            let removed = report.remove(i);
            if check_report(&report) {
                res += 1;
                continue 'outer;
            };
            report.insert(i, removed);
        }
    }

    println!("{res}")
}
