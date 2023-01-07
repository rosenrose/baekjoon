use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
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
        .filter_map(|&(ch, count)| (count == max_count).then(|| ch))
        .collect();

    max_counts.sort();

    println!("{}", String::from_iter(max_counts));
}
