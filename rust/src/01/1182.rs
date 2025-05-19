use std::io;

const MAX: usize = 20;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [n, s] = [(); 2].map(|_| input.next().unwrap());
    let mut nums = [0; MAX];

    for (i, num) in input.enumerate() {
        nums[i] = num;
    }

    let count: i32 = (1..=n as usize)
        .map(|i| combinations(0, 0, &mut [0; MAX], i, &nums, n as usize, s))
        .sum();

    println!("{count}");
}

fn combinations(
    depth: usize,
    start: usize,
    selected: &mut [usize; MAX],
    selected_len: usize,
    nums: &[i32],
    nums_len: usize,
    sum: i32,
) -> i32 {
    if depth == selected_len {
        return (selected[..selected_len]
            .iter()
            .map(|&i| nums[i])
            .sum::<i32>()
            == sum) as i32;
    }

    let takes = nums_len - (selected_len - 1);

    (start..depth + takes)
        .map(|i| {
            selected[depth] = i;
            combinations(
                depth + 1,
                i + 1,
                selected,
                selected_len,
                nums,
                nums_len,
                sum,
            )
        })
        .sum()
}
