use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    for i in 1..=input.next().unwrap() {
        let answers: Vec<_> = (1..=10).map(|j| (j, input.next().unwrap())).collect();
        let is_cheat = answers.iter().all(|&(j, a)| ((j - 1) % 5) + 1 == a);

        if is_cheat {
            println!("{i}");
        }
    }
}
