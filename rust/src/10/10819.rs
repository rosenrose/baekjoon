use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let n = input.next().unwrap();
    let nums: Vec<_> = input.collect();

    let max_sum = formula_max(&nums, n, &mut Vec::new());

    println!("{max_sum}");
}

fn formula_max(nums: &Vec<i32>, m: i32, selected: &mut Vec<usize>) -> u32 {
    if m == 0 {
        let formula: Vec<_> = selected.iter().map(|&i| nums[i]).collect();
        let sum: u32 = (1..formula.len())
            .map(|i| formula[i - 1].abs_diff(formula[i]))
            .sum();

        return sum;
    }

    (0..nums.len()).fold(0, |max, i| {
        if selected.contains(&i) {
            return max;
        }

        selected.push(i);

        let max = formula_max(nums, m - 1, selected).max(max);

        selected.pop();

        max
    })
}
