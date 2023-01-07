use std::collections::HashMap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let char_count = |s: &str| -> HashMap<_, _> {
        s.chars()
            .map(|c| (c, s.matches(c).count() as i32))
            .collect()
    };

    let a_counts = char_count(input.next().unwrap());
    let b_counts = char_count(input.next().unwrap());
    // println!("{a_count_map:?} {b_count_map:?}");

    let map_sub = |a: &HashMap<_, i32>, b: &HashMap<_, i32>| -> HashMap<_, _> {
        a.iter()
            .map(|(ch, count)| (*ch, (count - b.get(ch).unwrap_or(&0)).max(0)))
            .collect()
    };

    let a_delete_count: i32 = map_sub(&a_counts, &b_counts).values().sum();
    let b_delete_count: i32 = map_sub(&b_counts, &a_counts).values().sum();

    println!("{}", a_delete_count + b_delete_count);
}
