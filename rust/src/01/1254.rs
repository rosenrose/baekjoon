fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let s = buf.trim();
    let s_len = s.len();

    for size in (1..=s_len).rev() {
        if is_palindrome(&s[s_len - size..s_len]) {
            println!("{}", size + (s_len - size) * 2);
            return;
        }
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
