const INPUT: &'static str = include_str!("../../../input.txt");

#[derive(Debug, Clone, Copy)]
enum Block {
    Data { data: usize, len: usize },
    Empty(usize),
}
fn main() {
    let mut blocks = Vec::with_capacity(INPUT.len());
    let mut id = 0usize;
    for (i, c) in INPUT.char_indices() {
        let count = c as usize - '0' as usize;
        if i % 2 != 0 {
            if count == 0 {
                continue;
            };
            blocks.push(Block::Empty(count));
        } else {
            blocks.push(Block::Data {
                data: id,
                len: count,
            });
            id += 1;
        }
    }

    let mut p2 = blocks.len() - 1;
    // println!("{blocks:?}\n\n");

    'outer: while let Some(b) = blocks.get(p2).copied() {
        if let Block::Data { len, .. } = b {
            let mut p1 = 0;
            loop {
                if let Some(Block::Empty(empty_len)) = blocks.get(p1).copied() {
                    if len == empty_len {
                        blocks.swap(p1, p2);
                        continue 'outer;
                    } else if len < empty_len {
                        let diff = empty_len - len;
                        blocks.swap(p1, p2);
                        blocks[p2] = Block::Empty(len);
                        blocks.insert(p1 + 1, Block::Empty(diff));
                        continue 'outer;
                    }
                }

                p1 += 1;
                if p1 >= p2 {
                    break;
                }
            }
        }
        if let Some(new_p2) = p2.checked_sub(1) {
            p2 = new_p2
        } else {
            break;
        }
    }

    // println!(
    //     "{}",
    //     blocks.iter().fold(String::new(), |res, val| res
    //         + &match val {
    //             Block::Empty(x) => ".".repeat(*x),
    //             Block::Data { data, len } => data.to_string().repeat(*len),
    //         })
    // );

    let mut i = 0;
    let mut res = 0;

    for b in blocks {
        match b {
            Block::Data { data, len } => {
                for i in i..len + i {
                    res += i * data
                }
                i += len;
            }
            Block::Empty(l) => i += l,
        };
    }

    println!("{}", res);
}
