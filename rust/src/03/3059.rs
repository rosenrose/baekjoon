use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    for input in buf.lines().skip(1) {
        let sum: u32 = ('A'..='Z')
            .filter_map(|c| (!input.contains(c)).then(|| c as u32))
            .sum();

        println!("{sum}");
    }
}
