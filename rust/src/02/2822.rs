use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut score_indices: Vec<_> = buf
        .lines()
        .enumerate()
        .map(|(i, s)| (s.parse::<i32>().unwrap(), i + 1))
        .collect();

    let (third, _) = *score_indices.select_nth_unstable(2).1;

    let mut top_5: Vec<_> = score_indices.iter().filter(|&&(s, _)| s > third).collect();
    top_5.sort_by_key(|(_, i)| i);

    let sum: i32 = top_5.iter().map(|(s, _)| s).sum();

    println!("{sum}");

    for (_, i) in top_5 {
        print!("{i} ");
    }
}
