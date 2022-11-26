fn main() {
    let mut buf = String::new();

    loop {
        read_line(&mut buf);
        let password = buf.trim();

        if password == "end" {
            return;
        }

        println!(
            "<{password}> is {}",
            if is_acceptable(password) {
                "acceptable."
            } else {
                "not acceptable."
            }
        );
    }
}

fn is_acceptable(pwd: &str) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    if pwd.matches(vowels).count() == 0 {
        return false;
    }

    if pwd.split(vowels).any(|s| s.len() >= 3) {
        return false;
    }
    if pwd.split(|c| !vowels.contains(&c)).any(|s| s.len() >= 3) {
        return false;
    }

    if ('a'..='z')
        .filter_map(|c| {
            if c == 'e' || c == 'o' {
                None
            } else {
                Some(c.to_string().repeat(2))
            }
        })
        .any(|s| pwd.contains(&s))
    {
        return false;
    }

    true
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
