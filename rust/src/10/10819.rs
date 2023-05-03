use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let nums: Vec<_> = input.collect();

    let max_sum = permutations(0, &mut vec![0; n], &nums);

    println!("{max_sum}");
}

fn permutations(depth: usize, selected: &mut Vec<usize>, nums: &[i32]) -> u32 {
    if depth == selected.len() {
        let sum = (1..selected.len())
            .map(|i| nums[selected[i - 1]].abs_diff(nums[selected[i]]))
            .sum();

        return sum;
    }

    (0..nums.len())
        .map(|i| {
            if selected[..depth].contains(&i) {
                return 0;
            }

            selected[depth] = i;
            permutations(depth + 1, selected, nums)
        })
        .max()
        .unwrap()
}
