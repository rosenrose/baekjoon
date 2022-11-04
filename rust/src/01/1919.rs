use std::collections::HashMap;

fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let char_count = |s: &str| -> HashMap<char, usize> {
        s.chars()
            .map(|letter| {
                let count = s.chars().filter(|&c| c == letter).count();
                (letter, count)
            })
            .collect()
    };

    let a_count_map = char_count(buf.trim());
    read_line(&mut buf);

    let mut b_count_map = char_count(buf.trim());
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

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
