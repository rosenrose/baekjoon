use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    for n in input.skip(1) {
        for i in (0..).take_while(|i| (1 << i) <= n) {
            let shifted = 1 << i;

            if shifted & n == shifted {
                print!("{i} ");
            }
        }

        println!("");
    }
}
