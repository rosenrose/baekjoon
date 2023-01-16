use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for n in buf.lines().skip(1).flat_map(str::parse) {
        println!("{}", "=".repeat(n));
    }
}
