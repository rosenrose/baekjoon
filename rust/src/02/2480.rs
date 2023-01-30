fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let nums = parse_int_vec(&buf);
    let prize = match nums[..] {
        [a, b, c] if a == b && b == c => 10000 + a * 1000,
        [a, b, _] if a == b => 1000 + a * 100,
        [_, b, c] if b == c => 1000 + b * 100,
        [a, _, c] if c == a => 1000 + c * 100,
        _ => nums.iter().max().unwrap() * 100,
    };

    println!("{prize}");
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
