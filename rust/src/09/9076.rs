use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    for _ in 0..input.next().unwrap() {
        let mut scores: Vec<_> = (0..5).map(|_| input.next().unwrap()).collect();

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
