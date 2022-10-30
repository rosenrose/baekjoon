fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();

    let words = (0..n).map(|_| {
        read_line(&mut buf);
        buf.trim().to_string()
    });

    let group_words = words.filter(|word| is_group_word(word));

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

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
