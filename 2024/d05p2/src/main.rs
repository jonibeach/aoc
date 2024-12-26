use std::{cmp::Ordering, collections::HashMap};

const INPUT: &'static str = include_str!("../../../input.txt");

#[derive(Debug, Clone, Copy)]
enum Order {
    Before(usize),
    After(usize),
}

impl Order {
    fn inner(&self) -> usize {
        match self {
            Self::Before(i) => *i,
            Self::After(i) => *i,
        }
    }
}

fn check_update(
    update: &(usize, HashMap<usize, usize>, Vec<usize>),
    orderings: &HashMap<usize, Vec<Order>>,
) -> bool {
    let mut valid = true;

    for (num, num_index) in &update.1 {
        let temp = vec![];
        let num_orderings = orderings.get(&num).unwrap_or(&temp);

        for ord in num_orderings {
            if let Some(ordering_index) = update.1.get(&ord.inner()) {
                match ord {
                    Order::After(_) if num_index < ordering_index => valid = false,
                    Order::Before(_) if num_index > ordering_index => valid = false,
                    _ => {}
                }
            }
        }
    }

    valid
}

fn order_update_vec(v: &mut Vec<usize>, orderings: &HashMap<usize, Vec<Order>>) {
    v.sort_by(|a, b| {
        let mut ord = Ordering::Equal;

        if let Some(o) = orderings.get(a).and_then(|ords| {
            ords.iter().find(|ord| ord.inner() == *b).and_then(|ord| {
                Some(match ord {
                    Order::After(_) => Ordering::Greater,
                    Order::Before(_) => Ordering::Less,
                })
            })
        }) {
            ord = o;
        }

        ord
    });
}

fn main() {
    let mut orderings: HashMap<usize, Vec<Order>> = HashMap::new();
    let mut updates: Vec<(usize, HashMap<usize, usize>, Vec<usize>)> = Vec::new();

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
                    .clone()
                    .iter()
                    .enumerate()
                    .map(|(i, num)| (*num, i))
                    .collect(),
                update_vec,
            ))
        } else {
            let (num1_str, num2_str) = line.split_once("|").unwrap();
            let (num1, num2) = (num1_str.parse().unwrap(), num2_str.parse().unwrap());

            let num1_ordering = Order::Before(num2);
            let num2_ordering = Order::After(num1);

            orderings.entry(num1).or_insert(vec![]).push(num1_ordering);
            orderings.entry(num2).or_insert(vec![]).push(num2_ordering);
        }
    }

    let mut invalid_updates = Vec::new();

    for update in updates {
        if !check_update(&update, &orderings) {
            invalid_updates.push(update);
        }
    }

    let mut res = 0;

    for update in invalid_updates.iter_mut() {
        order_update_vec(&mut update.2, &orderings);
        res += update.0;
    }

    println!("{res}");
}
