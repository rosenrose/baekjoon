use std::io;

const MAX: usize = 100 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let mut seats = [false; MAX];
    let mut denied = 0;

    for num in input.skip(1) {
        if seats[num] {
            denied += 1;
        } else {
            seats[num] = true;
        }
    }

    println!("{denied}");
}
