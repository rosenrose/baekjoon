fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();

    let max = (0..n)
        .map(|_| {
            read_line(&mut buf);
            let nums = parse_int_vec(&buf);

            match nums[..] {
                [a, b, c] if a == b && b == c => 10000 + a * 1000,
                [a, b, _] if a == b => 1000 + a * 100,
                [_, b, c] if b == c => 1000 + b * 100,
                [a, _, c] if c == a => 1000 + c * 100,
                _ => nums.iter().max().unwrap() * 100,
            }
        })
        .max()
        .unwrap();

    println!("{max}");
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
