use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let n = input() as usize;
    let mut color_sizes: Vec<_> = (0..n).map(|i| (i, (input() - 1, input()))).collect();
    color_sizes.sort_unstable_by_key(|&(_, (_, size))| size);
    // println!("{color_sizes:?}");
    let mut results = vec![0; n];
    let mut sums = vec![0; n];
    let mut total_sum = 0;
    let mut last_infos = (0, vec![0; n]);

    for (i, &(idx, (color, size))) in color_sizes.iter().enumerate() {
        let prev_size = if i == 0 { 0 } else { color_sizes[i - 1].1 .1 };

        if prev_size < size {
            results[idx] = total_sum - sums[color as usize];

            last_infos.0 = total_sum;
            last_infos.1.copy_from_slice(&sums);
        } else {
            results[idx] = last_infos.0 - last_infos.1[color as usize];
        }

        sums[color as usize] += size;
        total_sum += size;
    }
    // println!("{results:?}");
    for sum in results {
        writeln!(output, "{sum}").unwrap();
    }

    print!("{output}");
}
