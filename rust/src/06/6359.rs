use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<i32>);

    for n in input.skip(1) {
        println!("{}", (1..).take_while(|i| i * i <= n).count());
    }
}
