use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let nums: Vec<usize> = buf.lines().flat_map(str::parse).collect();

    let avg = nums.iter().sum::<usize>() / nums.len();
    let mode = nums
        .iter()
        .max_by_key(|num| nums.iter().filter(|n| n == num).count())
        .unwrap();

    println!("{avg}\n{mode}");
}
