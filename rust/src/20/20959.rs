use std::collections::HashSet;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let integers: HashSet<_> = buf
        .trim()
        .split(char::is_alphabetic)
        .filter(|s| !s.is_empty())
        .collect();

    println!("{}", integers.len());
}
