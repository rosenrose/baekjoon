use std::cmp::Ordering;

fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let a = parse_int_vec(&buf);
    read_line(&mut buf);

    let b = parse_int_vec(&buf);

    let (a, b) = a
        .iter()
        .zip(b)
        .map(|(a, b)| match a.cmp(&b) {
            Ordering::Greater => (1, 0),
            Ordering::Equal => (0, 0),
            Ordering::Less => (0, 1),
        })
        .reduce(|(a1, b1), (a2, b2)| (a1 + a2, b1 + b2))
        .unwrap();

    match a.cmp(&b) {
        Ordering::Greater => println!("A"),
        Ordering::Equal => println!("D"),
        Ordering::Less => println!("B"),
    };
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
