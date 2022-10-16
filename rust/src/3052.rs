use std::collections::HashSet;

fn main() {
    let mut buf = String::new();
    let mut set = HashSet::new();

    for _ in 0..10 {
        read_line(&mut buf);
        set.insert(parse_int(&buf) % 42);
    }

    println!("{}", set.len());
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> i32 {
    buf.trim().parse().unwrap()
}
