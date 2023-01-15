use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let nums: Vec<_> = buf.lines().flat_map(str::parse::<usize>).collect();
    let avg = nums.iter().sum::<usize>() / nums.len();

    let counts = nums.iter().map(|&num| {
        let count = nums.iter().filter(|&n| *n == num).count();
        (num, count)
    });

    let (mode, _) = counts.max_by_key(|&(_, c)| c).unwrap();

    println!("{avg}\n{mode}");
}
