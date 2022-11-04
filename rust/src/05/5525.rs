fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: usize = buf.trim().parse().unwrap();
    read_line(&mut buf);
    read_line(&mut buf);

    let s = buf.trim();

    let pn = format!("I{}", "OI".repeat(n));
    let pn_chars: Vec<char> = pn.chars().collect();
    let partial_match = get_partial_match(&pn_chars);

    let mut count = 0;
    let mut start = 0;

    for s_char in s.chars() {
        while pn_chars[start] != s_char && start > 0 {
            start = partial_match[start - 1];
        }

        if pn_chars[start] == s_char {
            if start < pn.len() - 1 {
                start += 1;
            } else {
                count += 1;
                start = partial_match[start];
            }
        }
    }

    println!("{count}");
}

fn get_partial_match(chars: &Vec<char>) -> Vec<usize> {
    let mut partial_match = vec![0; chars.len()];
    let mut start = 0;

    for i in 1..chars.len() {
        while (chars[start] != chars[i]) && start > 0 {
            start = partial_match[start - 1];
        }

        if chars[start] == chars[i] {
            start += 1;
            partial_match[i] = start;
        }
    }

    partial_match
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
