use std::collections::HashSet;

fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();
    let words = parse_str_set_lines(&mut buf, n);

    let mut password = &String::new();

    for word in &words {
        let reversed: String = word.chars().rev().collect();

        if words.contains(&reversed) {
            password = word;
            break;
        }
    }

    let len = password.len();

    println!("{} {}", len, password.chars().nth(len / 2).unwrap());
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_str_set_lines(buf: &mut String, n: i32) -> HashSet<String> {
    (0..n)
        .map(|_| {
            read_line(buf);
            buf.trim().to_string()
        })
        .collect()
}
