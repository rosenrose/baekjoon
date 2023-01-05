fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    const M: i32 = 14579;
    let sum = |n: i32| (n * (n + 1)) / 2;

    if let [a, b] = parse_int_vec(&buf)[..] {
        println!("{}", (a..=b).fold(1, |acc, i| (acc * (sum(i) % M)) % M));
    }
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
