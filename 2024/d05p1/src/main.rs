use std::collections::HashMap;

const INPUT: &'static str = include_str!("../../../input.txt");

#[derive(Debug)]
enum Ordering {
    Before(usize),
    After(usize),
}

impl Ordering {
    fn inner(&self) -> usize {
        match self {
            Self::Before(i) => *i,
            Self::After(i) => *i,
        }
    }
}

fn main() {
    let mut orderings: HashMap<usize, Vec<Ordering>> = HashMap::new();
    let mut updates: Vec<(usize, HashMap<usize, usize>)> = Vec::new();

    let mut seen_empty = false;
    for line in INPUT.lines() {
        if line.is_empty() {
            seen_empty = true;
            continue;
        }

        if seen_empty {
            let update_vec: Vec<usize> = line.split(",").map(|str| str.parse().unwrap()).collect();
            let mid_value = update_vec[update_vec.len() / 2];

            updates.push((
                mid_value,
                update_vec
                    .iter()
                    .enumerate()
                    .map(|(i, num)| (*num, i))
                    .collect(),
            ))
        } else {
            let (num1_str, num2_str) = line.split_once("|").unwrap();
            let (num1, num2) = (num1_str.parse().unwrap(), num2_str.parse().unwrap());

            let num1_ordering = Ordering::Before(num2);
            let num2_ordering = Ordering::After(num1);

            orderings.entry(num1).or_insert(vec![]).push(num1_ordering);
            orderings.entry(num2).or_insert(vec![]).push(num2_ordering);
        }
    }

    let mut res = 0;

    for update in updates {
        let mut valid = true;

        for (num, num_index) in &update.1 {
            let temp = vec![];
            let num_orderings = orderings.get(&num).unwrap_or(&temp);
            // println!("ORDERINGS FOR {num} (index {num_index}) AR {num_orderings:?}");

            for ord in num_orderings {
                // println!("ORdERiNG {ord:?}, UPDATE: {update:?}");
                if let Some(ordering_index) = update.1.get(&ord.inner()) {
                    // println!("ORDERING {ord:?} applies ({}, {})", num, ord.inner());
                    match ord {
                        Ordering::After(_) if num_index < ordering_index => valid = false,
                        Ordering::Before(_) if num_index > ordering_index => valid = false,
                        _ => {}
                    }
                }
            }
        }

        if valid {
            res += update.0;
        }
    }

    println!("{res}");
}
