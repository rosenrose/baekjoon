use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let (n, s) = (input.next().unwrap(), input.next().unwrap());
    let nums: Vec<_> = input.collect();

    let count: i32 = (1..=n)
        .map(|i| combinations(0, 0, &mut vec![0; i as usize], s, &nums))
        .sum();

    println!("{count}");
}

fn combinations(
    depth: usize,
    start: usize,
    selected: &mut Vec<usize>,
    sum: i32,
    nums: &Vec<i32>,
) -> i32 {
    if depth == selected.len() {
        // println!("{selected:?}");
        return if selected.iter().map(|&i| nums[i]).sum::<i32>() == sum {
            1
        } else {
            0
        };
    }

    let takes = nums.len() - selected.len() + 1;

    (start..nums.len())
        .take(takes)
        .map(|i| {
            if selected[..depth].contains(&i) {
                return 0;
            }

            selected[depth] = i;
            combinations(depth + 1, i + 1, selected, sum, nums)
        })
        .sum()
}
