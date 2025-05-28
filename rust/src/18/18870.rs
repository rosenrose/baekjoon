use std::fmt::Write;
use std::io;

const MAX: usize = 1_000_000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let n = input.next().unwrap() as usize;
    let mut num_indices = [(0, 0); MAX];

    for (i, num) in input.enumerate() {
        num_indices[i] = (i, num);
    }

    num_indices[..n].sort_unstable_by_key(|&(_, num)| num);

    let mut compressed = [0; MAX];
    let mut unique_count = 0;

    for i in 1..n {
        let (index, num) = num_indices[i];

        if num != num_indices[i - 1].1 {
            unique_count += 1;
        }

        compressed[index] = unique_count;
    }

    for c in &compressed[..n] {
        write!(output, "{c} ").unwrap();
    }

    print!("{output}");
}
