use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    println!(
        "{}",
        match buf.matches('W').count() {
            5 | 6 => 1,
            3 | 4 => 2,
            1 | 2 => 3,
            _ => -1,
        }
    );
}
