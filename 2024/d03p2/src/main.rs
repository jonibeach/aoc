use regex::Regex;

const INPUT: &'static str = include_str!("../../../input.txt");

fn main() {
    let re = Regex::new(r"(mul\((\d+),(\d+)\)|(do\(\))|(don't\(\)))").unwrap();
    let mut res = 0;
    let mut doing = true;

    for c in re.find_iter(INPUT) {
        let mut str = c.as_str();
        
        match str {
            "do()" => {
                doing = true;
                continue;
            }
            "don't()" => {
                doing = false;
                continue;
            }
            _ => {}
        };

        // Remove leading mul( and trailing ).
        str = &str[4..str.len() - 1];

        let (num1, num2) = str.split_once(",").unwrap();
        let (num1, num2) = (num1.parse::<u64>().unwrap(), num2.parse::<u64>().unwrap());

        if doing {
            res += num1 * num2;
        }
    }

    println!("{res}")
}
