use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
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
        .filter_map(|&(title, count)| (count == max_count).then_some(title))
        .collect();

    best_sellers.sort();

    println!("{}", best_sellers[0]);
}
