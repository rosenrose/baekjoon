use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let n = input.next().unwrap() as usize;
    let mut num_indices: Vec<_> = input.enumerate().collect();

    num_indices.sort_unstable_by_key(|&(_, num)| num);

    let mut compressed = vec![0; n];
    let mut unique_count = 0;

    for i in 1..n {
        let (index, num) = num_indices[i];

        if num != num_indices[i - 1].1 {
            unique_count += 1;
        }

        compressed[index] = unique_count;
    }

    for c in compressed {
        write!(output, "{c} ").unwrap();
    }

    print!("{output}");
}
