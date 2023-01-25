use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().next_back().unwrap().to_owned();

    while input.matches('s').count() != input.matches('t').count() {
        input.remove(0);
    }

    println!("{input}");
}
