use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for input in buf.lines().skip(1) {
        let words: Vec<_> = input.split(' ').collect();

        println!("{}", [&words[2..], &words[..2]].concat().join(" "));
    }
}
