fn main() {
    let mut buf = String::new();

    let burgers = parse_int_vec_lines(&mut buf, 3);
    let drinks = parse_int_vec_lines(&mut buf, 2);

    let prices = (0..burgers.len() * drinks.len()).map(|i| burgers[i % 3] + drinks[i / 3] - 50);

    println!("{}", prices.min().unwrap());
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> i32 {
    buf.trim().parse().unwrap()
}

fn parse_int_vec_lines(buf: &mut String, n: i32) -> Vec<i32> {
    (0..n)
        .map(|_| {
            read_line(buf);
            parse_int(buf)
        })
        .collect()
}
