fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();
    let (mut q1, mut q2, mut q3, mut q4, mut axis) = (0, 0, 0, 0, 0);

    for _ in 0..n {
        read_line(&mut buf);

        match parse_int_vec(&buf)[..] {
            [x, y] if x > 0 && y > 0 => q1 += 1,
            [x, y] if x < 0 && y > 0 => q2 += 1,
            [x, y] if x < 0 && y < 0 => q3 += 1,
            [x, y] if x > 0 && y < 0 => q4 += 1,
            _ => axis += 1,
        };
    }

    println!("Q1: {q1}\nQ2: {q2}\nQ3: {q3}\nQ4: {q4}\nAXIS: {axis}");
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
