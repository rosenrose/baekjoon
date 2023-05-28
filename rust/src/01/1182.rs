use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let (n, s) = (input.next().unwrap() as usize, input.next().unwrap());
    let nums: Vec<_> = input.collect();

    let count: i32 = (1..=n)
        .map(|i| combinations(0, 0, &mut vec![0; i], &nums, s))
        .sum();

    println!("{count}");
}

fn combinations(
    depth: usize,
    start: usize,
    selected: &mut Vec<usize>,
    nums: &[i32],
    sum: i32,
) -> i32 {
    if depth == selected.len() {
        return i32::from(selected.iter().map(|&i| nums[i]).sum::<i32>() == sum);
    }

    let takes = nums.len() - (selected.len() - 1);

    (start..depth + takes)
        .map(|i| {
            selected[depth] = i;
            combinations(depth + 1, i + 1, selected, nums, sum)
        })
        .sum()
}
