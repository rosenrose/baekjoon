use std::fmt::Write;
use std::io;

const MAX: usize = 200_000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let n = input() as usize;
    let mut color_sizes: Vec<_> = (0..n).map(|i| (i, (input() - 1, input()))).collect();
    color_sizes.sort_unstable_by_key(|&(_, (_, size))| size);
    // println!("{color_sizes:?}");
    let mut results = [0; MAX];
    let mut sums = [0; MAX];
    let mut total_sum = 0;
    let mut cursor = 0;

    for (i, &(num, (color, size))) in color_sizes.iter().enumerate() {
        results[num] = total_sum - sums[color as usize];

        if i < n - 1 {
            let next_size = color_sizes[i + 1].1 .1;

            if size == next_size {
                continue;
            }
        }

        for &(_, (c, s)) in &color_sizes[cursor..i] {
            sums[c as usize] += s;
            total_sum += s;
        }

        sums[color as usize] += size;
        total_sum += size;
        cursor = i + 1;
    }

    for sum in &results[..n] {
        writeln!(output, "{sum}").unwrap();
    }

    print!("{output}");
}
