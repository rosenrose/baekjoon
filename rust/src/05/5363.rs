use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for input in buf.lines().skip(1) {
        let mut words: Vec<_> = input.split(' ').collect();
        words.rotate_left(2);

        println!("{}", words.join(" "));
    }
}
