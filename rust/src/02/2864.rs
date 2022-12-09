fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut input = buf.split_whitespace();
    let (a, b) = (input.next().unwrap(), input.next().unwrap());

    let five_to_six = |s: &str| s.replace("5", "6");
    let six_to_five = |s: &str| s.replace("6", "5");

    let (min_a, max_a) = (parse_int(six_to_five(a)), parse_int(five_to_six(a)));
    let (min_b, max_b) = (parse_int(six_to_five(b)), parse_int(five_to_six(b)));

    println!("{} {}", min_a + min_b, max_a + max_b);
}

fn parse_int(buf: String) -> i32 {
    buf.parse().unwrap()
}
