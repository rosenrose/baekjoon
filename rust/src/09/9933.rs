use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let words: Vec<_> = buf.lines().skip(1).map(str::to_owned).collect();

    for word in words.iter() {
        let reversed: String = word.chars().rev().collect();

        if words.contains(&reversed) {
            let len = word.len();

            println!("{} {}", len, word.chars().nth(len / 2).unwrap());
            break;
        }
    }
}
