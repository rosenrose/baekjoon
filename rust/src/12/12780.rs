use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let (h, n) = (input.next().unwrap(), input.next().unwrap());

    println!("{}", h.matches(n).count());
}
