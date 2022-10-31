use std::collections::HashSet;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let word = buf.trim().to_uppercase();
    let letters: HashSet<char> = word.chars().collect();

    let (mut most_used, mut max) = (' ', 0);
    let mut is_duplicate = false;

    for letter in letters {
        let count = word.chars().filter(|&c| c == letter).count();

        if count == max {
            is_duplicate = true;
        }

        if count > max {
            is_duplicate = false;
            (most_used, max) = (letter, count);
        }
    }

    if is_duplicate {
        println!("?");
        return;
    }

    println!("{most_used}");
}
