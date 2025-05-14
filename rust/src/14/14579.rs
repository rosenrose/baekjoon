fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [a, b] = parse_int_vec(&buf)[..] else {
        return;
    };
    let sum = |n: i32| (n * (n + 1)) / 2;
    const M: i32 = 14579;

    println!("{}", (a..=b).fold(1, |acc, i| (acc * (sum(i) % M)) % M));
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
