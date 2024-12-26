const INPUT: &'static str = include_str!("../../../input.txt");
fn main() {
    let mut blocks = Vec::with_capacity(INPUT.len());
    let mut id = 0usize;
    for (i, c) in INPUT.char_indices() {
        let count = c as usize - '0' as usize;
        if i % 2 != 0 {
            blocks.extend(vec![None; count as usize]);
        } else {
            blocks.extend(vec![Some(id); count]);
            id += 1;
        }
    }

    let mut p1 = 0;
    let mut p2 = blocks.len() - 1;

    while let Some(b) = blocks.get(p1) {
        if b == &None {
            loop {
                if let Some(Some(_)) = blocks.get(p2) {
                    blocks.swap(p1, p2);
                    p2 -= 1;
                    break;
                }
                p2 -= 1;
                if p1 >= p2 {
                    break;
                }
            }
        }

        if p1 >= p2 {
            break;
        }
        p1 += 1
    }

    let res = blocks
        .iter()
        .enumerate()
        .map_while(|(i, v)| if let Some(v) = v { Some((i, v)) } else { None })
        .fold(0, |res, (i, v)| res + i * v);
    
    println!("{res}");
}
