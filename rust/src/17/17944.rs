fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [n, t] = parse_int_vec(&buf)[..] {
        let (mut num, mut delta) = (1, 1);

        for _ in 0..t - 1 {
            num += delta;

            if num == n * 2 || num == 1 {
                delta *= -1;
            }
        }

        println!("{num}");
    }
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}