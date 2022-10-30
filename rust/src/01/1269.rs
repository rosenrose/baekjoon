use std::collections::HashSet;

fn main() {
    let mut buf = String::new();
    read_line(&mut buf);
    read_line(&mut buf);

    let parse_int = |s: &str| s.parse::<i32>().unwrap();

    let mut diff: HashSet<i32> = buf.split_whitespace().map(parse_int).collect();
    read_line(&mut buf);

    for num in buf.split_whitespace().map(parse_int) {
        match diff.contains(&num) {
            true => diff.remove(&num),
            false => diff.insert(num),
        };
    }

    println!("{}", diff.len());
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
