use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let words: Vec<_> = buf.lines().skip(1).map(str::to_owned).collect();

    for word in &words {
        let reversed: String = word.chars().rev().collect();

        if words.contains(&reversed) {
            let len = word.len();

            println!("{} {}", len, word.chars().nth(len / 2).unwrap());
            break;
        }
    }
}
