use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let titles: Vec<_> = buf.lines().skip(1).collect();

    let mut max_count = 0;
    let counts: Vec<_> = titles
        .iter()
        .map(|title| {
            let count = titles.iter().filter(|&t| t == title).count();
            max_count = count.max(max_count);

            (title, count)
        })
        .collect();

    let mut best_sellers: Vec<_> = counts
        .iter()
        .filter_map(|&(title, count)| (count == max_count).then(|| title))
        .collect();

    best_sellers.sort();

    println!("{}", best_sellers[0]);
}
