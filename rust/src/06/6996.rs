use std::collections::HashMap;

fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();

    let char_count = |s: &str| -> HashMap<char, usize> {
        s.chars()
            .map(|letter| {
                let count = s.chars().filter(|&c| c == letter).count();
                (letter, count)
            })
            .collect()
    };

    for _ in 0..n {
        read_line(&mut buf);

        if let [a, b] = parse_str_vec(&buf)[..] {
            let a_count_map = char_count(a);
            let b_count_map = char_count(b);

            println!(
                "{a} & {b} are {}",
                if a_count_map == b_count_map {
                    "anagrams."
                } else {
                    "NOT anagrams."
                }
            );
        }
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_str_vec(buf: &String) -> Vec<&str> {
    buf.split_whitespace().collect()
}
