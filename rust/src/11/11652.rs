use std::collections::HashMap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().map(|s| s.parse::<i64>().unwrap());

    let mut max_count = 1;
    let mut counts = HashMap::new();

    for num in input.skip(1) {
        counts
            .entry(num)
            .and_modify(|c| {
                *c += 1;
                max_count = max_count.max(*c);
            })
            .or_insert(1);
    }

    let (min, _) = counts
        .iter()
        .filter(|(_, &v)| v == max_count)
        .min_by_key(|(&k, _)| k)
        .unwrap();

    println!("{min}");
}
