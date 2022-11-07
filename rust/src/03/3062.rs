fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n = parse_int(&buf);

    for _ in 0..n {
        read_line(&mut buf);
        let num = parse_int(&buf);

        let sum = num
            + num
                .to_string()
                .chars()
                .rev()
                .collect::<String>()
                .parse::<i32>()
                .unwrap();

        println!(
            "{}",
            if is_palindrome(&sum.to_string()[..]) {
                "YES"
            } else {
                "NO"
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

fn parse_int(buf: &String) -> i32 {
    buf.trim().parse().unwrap()
}
