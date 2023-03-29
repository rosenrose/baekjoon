use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    for _ in 0..input.next().unwrap() {
        let mut scores: Vec<_> = input.by_ref().take(5).collect();

        scores.sort();
        scores.pop();
        scores.remove(0);

        if scores[0].abs_diff(*scores.last().unwrap()) >= 4 {
            println!("KIN");
            continue;
        }

        println!("{}", scores.iter().sum::<i32>());
    }
}
