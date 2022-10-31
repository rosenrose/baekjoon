use std::cmp::Ordering;

fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let month = parse_int(&buf);
    read_line(&mut buf);

    let date = parse_int(&buf);

    match (month, date).cmp(&(2, 18)) {
        Ordering::Less => println!("Before"),
        Ordering::Equal => println!("Special"),
        Ordering::Greater => println!("After"),
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> i32 {
    buf.trim().parse().unwrap()
}
