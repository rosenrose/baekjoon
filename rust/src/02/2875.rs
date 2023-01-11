fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [mut girls, mut boys, k] = parse_int_vec(&buf)[..] else { return };

    for _ in 0..k {
        if girls >= boys * 2 {
            girls -= 1;
        } else {
            boys -= 1;
        }
    }

    let teams = if girls >= boys * 2 { boys } else { girls / 2 };

    println!("{teams}");
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
