fn main() {
    let mut buf = String::new();

    loop {
        read_line(&mut buf);

        if buf.trim().chars().nth(0).unwrap() == '0' {
            return;
        }

        println!(
            "{}",
            if is_palindrome(buf.trim()) {
                "yes"
            } else {
                "no"
            }
        );
    }
}

fn is_palindrome(word: &str) -> bool {
    let len = word.len();

    if len <= 1 {
        return true;
    }

    if word.chars().nth(0) != word.chars().nth(len - 1) {
        return false;
    }

    is_palindrome(&word[1..len - 1])
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
