use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().next_back().unwrap();

    for sentence in input.split_terminator(['.', '?', '!']) {
        let names = sentence
            .split(' ')
            .filter(|word| {
                word.starts_with(char::is_uppercase) && word[1..].chars().all(char::is_lowercase)
            })
            .count();

        println!("{names}");
    }
}
