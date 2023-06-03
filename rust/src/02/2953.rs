use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut scores = [0; 5];

    for score in &mut scores {
        *score = input.by_ref().take(4).sum();
    }

    let (idx, max_score) = scores
        .iter()
        .enumerate()
        .max_by_key(|&(_, score)| score)
        .unwrap();

    println!("{} {max_score}", idx + 1);
}
