use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for n in buf.lines().skip(1).flat_map(str::parse) {
        println!("{}", (1..=n).step_by(2).sum::<i32>());
    }
}
