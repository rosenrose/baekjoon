use std::cmp::Ordering;

fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();
    let (mut score_a, mut score_b) = (100, 100);

    for _ in 0..n {
        read_line(&mut buf);

        if let [a, b] = parse_int_vec(&buf)[..] {
            match a.cmp(&b) {
                Ordering::Less => score_a -= b,
                Ordering::Equal => (),
                Ordering::Greater => score_b -= a,
            };
        }
    }

    println!("{score_a}\n{score_b}");
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
