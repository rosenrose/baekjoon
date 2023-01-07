use std::io;

fn main() {
    print!("{}", io::read_to_string(io::stdin()).unwrap());
}
