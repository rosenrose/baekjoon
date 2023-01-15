use std::cmp::Ordering;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [a, b, c] = parse_int_vec(&buf)[..] else { return };
    let mut tastes = [a, b, c, a * b, a * c, b * c, a * b * c];

    tastes.sort_by(|a, b| match (a % 2, b % 2) {
        (1, 0) => Ordering::Greater,
        (0, 1) => Ordering::Less,
        _ => a.cmp(b),
    });

    println!("{}", tastes.iter().last().unwrap());
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
