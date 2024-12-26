const INPUT: &'static str = include_str!("../../../input.txt");

fn calculate(target: usize, curr: usize, nums: &[usize]) -> bool {
    // println!("TARG: {target}, CURR {curr}, NUMS {nums:?}");

    if curr == target {
        return true;
    }

    if let Some(num) = nums.first() {
        let sum = calculate(target, curr + num, &nums[1..]);
        
        if curr == 0 {
            return sum;
        };
        
        let mul = calculate(target, curr * num, &nums[1..]);
        
        return sum | mul
    }

    false
}

fn main() {
    let mut res = 0;
    for l in INPUT.lines() {
        let (val, nums) = l.split_once(":").unwrap();
        let val = val.parse::<usize>().unwrap();
        let nums = nums
            .split(" ")
            .filter_map(|str| str.parse::<usize>().ok())
            .collect::<Vec<usize>>();

        if calculate(val, 0, &nums) {
            res += val
        }
    }

    println!("{res}");
}
