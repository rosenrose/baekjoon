fn main() {
    let mut buf = String::new();
    read_line(&mut buf);
    read_line(&mut buf);

    let mut a = parse_int_vec(&buf);
    read_line(&mut buf);

    let mut b = parse_int_vec(&buf);

    a.sort();
    b.sort_by(|x, y| y.cmp(x));

    let product_sum: i32 = a.iter().zip(b).map(|(a, b)| a * b).sum();

    println!("{product_sum}");
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
