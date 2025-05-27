use std::io;

const MAX: usize = 9;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i64>);

    let [n, k] = [(); 2].map(|_| input.next().unwrap());
    let mut nums = [0; 3];

    for (i, num) in input.enumerate() {
        nums[i] = num;
    }

    let num = (1..=n.ilog10() + 1)
        .flat_map(|i| product(0, &mut [0; MAX][..i as usize], &nums[..k as usize], n))
        .max()
        .unwrap();

    println!("{num}");
}

fn product(depth: usize, selected: &mut [i64], nums: &[i64], max_num: i64) -> Option<i64> {
    if depth == selected.len() {
        let result = selected.iter().fold(0, |acc, num| acc * 10 + num);

        return (result <= max_num).then_some(result);
    }

    nums.iter()
        .flat_map(|&num| {
            selected[depth] = num;
            product(depth + 1, selected, nums, max_num)
        })
        .max()
}
