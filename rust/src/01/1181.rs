use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut words: Vec<_> = buf.lines().skip(1).collect();

    words.sort_unstable_by_key(|&w| (w.len(), w));
    words.dedup();

    println!("{}", words.join("\n"));
}
