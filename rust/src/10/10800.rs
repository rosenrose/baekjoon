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
    let mut temp_color_sizes = Vec::new();

    for (i, &(num, (color, size))) in color_sizes.iter().enumerate() {
        results[num] = total_sum - sums[color as usize];

        if i == n - 1 {
            continue;
        }

        let next_size = color_sizes[i + 1].1 .1;

        if size < next_size {
            for &(c, s) in temp_color_sizes.iter() {
                sums[c as usize] += s;
                total_sum += s;
            }
            temp_color_sizes.clear();

            sums[color as usize] += size;
            total_sum += size;
        } else {
            temp_color_sizes.push((color, size));
        }
    }

    for sum in results {
        writeln!(output, "{sum}").unwrap();
    }

    print!("{output}");
}
