use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let nums: Vec<_> = input.skip(1).collect();
    let Some(prev) = prev_permutation(nums) else {
        println!("-1");
        return;
    };

    for num in prev {
        print!("{num} ");
    }
}

fn prev_permutation(mut nums: Vec<i32>) -> Option<Vec<i32>> {
    let len = nums.len();

    let i = (1..len).rfind(|&i| nums[i - 1] > nums[i])?;
    let j = (i..len).rfind(|&j| nums[j] < nums[i - 1]).unwrap();

    nums.swap(i - 1, j);
    (&mut nums[i..]).sort_by(|a, b| a.cmp(b).reverse());

    Some(nums)
}
