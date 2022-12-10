use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let nums: Vec<_> = buf.lines().map(|s| s.parse::<usize>().unwrap()).collect();
    let avg = nums.iter().sum::<usize>() / nums.len();

    let counts = nums.iter().map(|&num| {
        let count = nums.iter().filter(|&n| *n == num).count();
        (num, count)
    });

    let (mode, _) = counts.max_by_key(|&(_, c)| c).unwrap();

    println!("{avg}\n{mode}");
}
