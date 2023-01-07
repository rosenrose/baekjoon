use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let group_words = buf.lines().skip(1).filter(|word| is_group_word(word));

    println!("{}", group_words.count());
}

fn is_group_word(word: &str) -> bool {
    if word.len() <= 2 {
        return true;
    }

    let mut current = word.chars().nth(0).unwrap();
    let mut check_finished = Vec::new();

    for next in word.chars().skip(1) {
        if next == current {
            continue;
        }

        if check_finished.contains(&next) {
            return false;
        }

        check_finished.push(current);
        current = next;
    }

    true
}
