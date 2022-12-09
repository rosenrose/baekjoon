use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let words: Vec<_> = buf.lines().collect();
    let longest = words.iter().map(|word| word.len()).max().unwrap();

    for i in 0..longest {
        for word in words.iter() {
            if let Some(c) = word.chars().nth(i) {
                print!("{c}");
            }
        }
    }
}
