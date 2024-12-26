const INPUT: &'static str = include_str!("../../../input.txt");

fn calculate(target: usize, curr: usize, nums: &[usize]) -> bool {
    if nums.is_empty() {
        return curr == target;
    }

    let sum = calculate(target, curr + nums[0], &nums[1..]);
    if curr == 0 {
        return sum;
    };

    let mul = calculate(target, curr * nums[0], &nums[1..]);
    let mut res = sum | mul;

    let digits = (nums[0] as f64).log10().floor() as u32 + 1;
    let curr = curr * 10_usize.pow(digits) + nums[0];
    let concat = calculate(target, curr, &nums[1..]);
    res |= concat;

    res
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
