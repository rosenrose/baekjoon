use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<i32>);

    for k in input.skip(1) {
        println!("{}", (1 << k) - 1);
    }
}
