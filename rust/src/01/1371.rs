use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut max_count = 0;
    let counts: Vec<(char, usize)> = ('a'..='z')
        .map(|letter| {
            let count = buf.matches(letter).count();
            if count > max_count {
                max_count = count;
            }

            (letter, count)
        })
        .collect();

    let mut max_counts: Vec<String> = counts
        .iter()
        .filter(|(_, count)| *count == max_count)
        .map(|(c, _)| c.to_string())
        .collect();

    max_counts.sort();

    println!("{}", max_counts.concat());
}
