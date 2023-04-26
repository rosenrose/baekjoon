use std::collections::HashMap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let a: Vec<_> = input.by_ref().take(n).collect();

    let mut sum_counts = HashMap::with_capacity(n);
    sum_counts.insert(0, 1_i64);

    let mut sum = 0;
    let mut count = 0;

    for (i, num) in input.enumerate() {
        sum += a[i] - num;
        count += sum_counts.get(&sum).unwrap_or(&0);

        sum_counts.entry(sum).and_modify(|c| *c += 1).or_insert(1);
    }

    println!("{count}");
}
// a[j] - a[i] = b[j] - b[i]
// a[j] - a[i] - b[j] + b[i] = 0
// (a[j] - b[j]) - (a[i] - b[i]) = 0
// c[j] - c[i] = 0
