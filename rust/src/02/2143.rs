use std::collections::HashMap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let t = input.next().unwrap();
    let n = input.next().unwrap() as usize;
    let mut a_sums = vec![0; n + 1];

    for (i, num) in input.by_ref().take(n).enumerate() {
        a_sums[i + 1] = a_sums[i] + num;
    }

    let m = input.next().unwrap() as usize;
    let mut b_sums = vec![0; m + 1];

    for (i, num) in input.enumerate() {
        b_sums[i + 1] = b_sums[i] + num;
    }
    // println!("{a_sums:?} {b_sums:?}");
    let mut a_counts = HashMap::with_capacity(n * n / 2);
    let mut b_counts = HashMap::with_capacity(m * m / 2);

    for i in 1..=n {
        for j in 0..i {
            a_counts
                .entry(a_sums[i] - a_sums[j])
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }
    }
    for i in 1..=m {
        for j in 0..i {
            b_counts
                .entry(b_sums[i] - b_sums[j])
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }
    }
    // println!("{a:?} {b:?}");
    let count: i64 = a_counts
        .iter()
        .map(|(a_num, a_count)| a_count * b_counts.get(&(t - a_num)).unwrap_or(&0))
        .sum();

    println!("{count}");
}
