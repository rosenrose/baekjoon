fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let b1 = i64::from_str_radix(buf.trim(), 2).unwrap();
    read_line(&mut buf);

    let b2 = i64::from_str_radix(buf.trim(), 2).unwrap();

    println!("{:b}", b1 * b2);
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
