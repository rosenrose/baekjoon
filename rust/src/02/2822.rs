use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut scores: Vec<_> = buf
        .lines()
        .enumerate()
        .map(|(i, s)| (s.parse::<i32>().unwrap(), i + 1))
        .collect();
    scores.sort_by_key(|&(s, _)| s);

    let (third, _) = scores[2];

    let mut top_5: Vec<_> = scores.iter().filter(|&&(s, _)| s > third).collect();
    top_5.sort_by_key(|(_, i)| i);

    let sum: i32 = top_5.iter().map(|(s, _)| s).sum();

    println!("{sum}");
    for (_, i) in top_5 {
        print!("{i} ");
    }
}
