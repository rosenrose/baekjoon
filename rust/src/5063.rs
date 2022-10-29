use std::cmp::Ordering;

fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();

    for _ in 0..n {
        read_line(&mut buf);

        if let [r, e, c] = parse_int_vec(&buf)[..] {
            match r.cmp(&(e - c)) {
                Ordering::Less => println!("advertise"),
                Ordering::Equal => println!("does not matter"),
                Ordering::Greater => println!("do not advertise"),
            };
        }
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
