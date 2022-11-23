fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();
    let mut nums: Vec<String> = (0..n)
        .flat_map(|_| {
            read_line(&mut buf);

            buf.trim()
                .split(char::is_alphabetic)
                .filter_map(|s| {
                    if s.is_empty() {
                        return None;
                    }
                    if s.chars().all(|c| c == '0') {
                        return Some("0".to_string());
                    }

                    let mut s = s.to_string();

                    loop {
                        match s.chars().nth(0) {
                            Some('0') => s.remove(0),
                            _ => break,
                        };
                    }

                    Some(s)
                })
                .collect::<Vec<_>>()
        })
        .collect();

    nums.sort_by(|a, b| {
        if a.len() == b.len() {
            a.cmp(b)
        } else {
            a.len().cmp(&b.len())
        }
    });

    for num in nums {
        println!("{num}");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
