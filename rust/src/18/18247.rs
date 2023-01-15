use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    for (n, m) in (0..input()).map(|_| (input(), input())) {
        if n < 12 || m < 4 {
            println!("-1");
            continue;
        }

        println!("{}", 4 + 11 * m);
    }
}
