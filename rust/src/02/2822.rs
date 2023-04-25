use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut idx_scores: Vec<_> = buf
        .lines()
        .flat_map(str::parse::<i32>)
        .enumerate()
        .collect();

    let (_, third) = *idx_scores.select_nth_unstable_by_key(2, |&(_, s)| s).1;

    let mut sum = 0;
    let mut top_5: Vec<_> = idx_scores
        .iter()
        .filter_map(|&(i, s)| {
            (s > third).then(|| {
                sum += s;
                i + 1
            })
        })
        .collect();
    top_5.sort();

    println!("{sum}");

    for i in top_5 {
        print!("{i} ");
    }
}
