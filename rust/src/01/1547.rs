fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();
    let mut ball = 1;

    for _ in 0..n {
        read_line(&mut buf);

        ball = match (ball, &parse_int_vec(&buf)[..]) {
            (1, [1, 3] | [3, 1]) | (2, [2, 3] | [3, 2]) => 3,
            (1, [1, 2] | [2, 1]) | (3, [2, 3] | [3, 2]) => 2,
            (2, [1, 2] | [2, 1]) | (3, [1, 3] | [3, 1]) => 1,
            _ => continue,
        };
    }

    println!("{ball}");
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
