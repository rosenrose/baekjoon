use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
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
