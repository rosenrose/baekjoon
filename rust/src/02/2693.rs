use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    for _ in 0..input.next().unwrap() {
        let mut scores: Vec<_> = (0..10).map(|_| input.next().unwrap()).collect();

        writeln!(output, "{}", scores.select_nth_unstable(7).1).unwrap();
    }

    print!("{output}");
}
