fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [n, k] = parse_int_vec(&buf)[..] else { return };
    let max = (1..=k)
        .flat_map(|i| {
            (n * i)
                .to_string()
                .chars()
                .rev()
                .collect::<String>()
                .parse::<i32>()
        })
        .max()
        .unwrap();

    println!("{max}");
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
