use std::collections::HashMap;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let char_count = |s: &str| -> HashMap<char, usize> {
        s.chars().map(|c| (c, s.matches(c).count())).collect()
    };

    let a_count_map = char_count(input.next().unwrap());
    let mut b_count_map = char_count(input.next().unwrap());
    // println!("{a_count_map:?} {b_count_map:?}");

    let map_sub = |a: &mut HashMap<_, usize>, b: &HashMap<_, usize>| {
        b.iter().for_each(|(&key, &value)| {
            a.entry(key).and_modify(|v| *v = v.saturating_sub(value));
        });
    };
    let mut temp = a_count_map.clone();

    map_sub(&mut temp, &b_count_map);
    let a_delete_count: usize = temp.values().sum();

    map_sub(&mut b_count_map, &a_count_map);
    let b_delete_count: usize = b_count_map.values().sum();

    println!("{}", a_delete_count + b_delete_count);
}
