use std::collections::HashSet;

fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n = parse_int(&buf);

    let words: HashSet<String> = (0..n)
        .map(|_| {
            read_line(&mut buf);
            buf.trim().to_string()
        })
        .collect();

    let mut words: Vec<String> = words.into_iter().collect();

    words.sort_by(|a, b| {
        if a.len() == b.len() {
            a.cmp(b)
        } else {
            a.len().cmp(&b.len())
        }
    });

    println!("{}", words.join("\n"));
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> i32 {
    buf.trim().parse().unwrap()
}
