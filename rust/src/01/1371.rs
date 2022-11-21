use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut max_count = 0;
    let counts: Vec<_> = ('a'..='z')
        .map(|letter| {
            let count = buf.matches(letter).count();
            max_count = count.max(max_count);

            (letter, count)
        })
        .collect();

    let mut max_counts: Vec<_> = counts
        .iter()
        .filter_map(|&(ch, count)| if count == max_count { Some(ch) } else { None })
        .collect();

    max_counts.sort();

    println!("{}", String::from_iter(max_counts));
}
