fn main() {
    let mut buf = String::new();

    loop {
        read_line(&mut buf);

        if buf.trim().chars().nth(0).unwrap() == '0' {
            return;
        }

        println!("{}", is_palindrome(buf.trim()));
    }
}

fn is_palindrome(word: &str) -> &str {
    let len = word.len();

    if len <= 1 {
        return "yes";
    }

    if word.chars().nth(0) != word.chars().nth(len - 1) {
        return "no";
    }

    is_palindrome(&word[1..len - 1])
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
