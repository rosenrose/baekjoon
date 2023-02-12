use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let word = buf.lines().next().unwrap();

    println!("{}", buf.matches(word).count() - 1);
}
