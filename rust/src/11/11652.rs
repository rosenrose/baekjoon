use std::collections::HashMap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(str::parse::<i64>);

    let n = input.next().unwrap() as usize;
    let mut counts = HashMap::with_capacity(n);
    let mut max_count = 1;

    for num in input {
        counts
            .entry(num)
            .and_modify(|c| {
                *c += 1;
                max_count = max_count.max(*c);
            })
            .or_insert(1);
    }

    let min = counts
        .iter()
        .filter_map(|(k, &v)| (v == max_count).then_some(k))
        .min()
        .unwrap();

    println!("{min}");
}
