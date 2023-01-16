use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for input in buf.lines().skip(1) {
        let sum: u32 = ('A'..='Z')
            .filter_map(|c| (!input.contains(c)).then_some(c as u32))
            .sum();

        println!("{sum}");
    }
}
