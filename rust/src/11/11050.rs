fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [n, k] = parse_int_vec(&buf)[..] else { return };

    println!("{}", combination_num(n, k));
}

fn combination_num(n: i32, r: i32) -> i32 {
    if n == r || r == 0 {
        return 1;
    }
    // combination_num(n - 1, r - 1) + combination_num(n - 1, r)
    (n - r + 1..=n).product::<i32>() / (1..=r).product::<i32>()
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
