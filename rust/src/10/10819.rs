use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let nums: Vec<_> = input.collect();

    let max_sum = formula_max(0, &mut vec![0; n], &nums);

    println!("{max_sum}");
}

fn formula_max(depth: usize, selected: &mut Vec<usize>, nums: &Vec<i32>) -> u32 {
    if depth == selected.len() {
        let sum = (1..selected.len())
            .map(|i| nums[selected[i - 1]].abs_diff(nums[selected[i]]))
            .sum();

        return sum;
    }

    (0..nums.len()).fold(0, |max, i| {
        if selected[..depth].contains(&i) {
            return max;
        }

        selected[depth] = i;

        let result = formula_max(depth + 1, selected, nums);

        result.max(max)
    })
}
