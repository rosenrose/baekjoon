use std::collections::HashSet;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let string = buf.trim().to_string();
    let mut substr = HashSet::new();

    for i in 1..=string.len() {
        for j in 0..=string.len() - i {
            substr.insert(&string[j..j + i]);
        }
    }

    println!("{}", substr.len());
}
