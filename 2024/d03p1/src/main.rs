use regex::Regex;

const INPUT: &'static str = include_str!("../../../input.txt");


fn main() {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut res = 0;
    for c in re.captures_iter(INPUT) {
        let (_, [num1, num2]) = c.extract();
        let (num1, num2) = (num1.parse::<u64>().unwrap(), num2.parse::<u64>().unwrap());
        res += num1 * num2;
    }
    
    println!("{res}")
}
