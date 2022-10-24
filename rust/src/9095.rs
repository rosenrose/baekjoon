fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n = parse_int(&buf);

    for _ in 0..n {
        read_line(&mut buf);
        let num = parse_int(&buf);

        println!("{}", permutation_123(num));
    }
}

fn permutation_123(num: i32) -> i32 {
    if num <= 2 {
        return num;
    }
    if num == 3 {
        return 4;
    }
    //(num - 3..=num - 1).map(|n| permutation_123(n)).sum()
    let (mut a, mut b, mut c) = (1, 2, 4);

    for _ in 4..=num {
        (a, b, c) = (b, c, a + b + c);
    }

    c
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> i32 {
    buf.trim().parse().unwrap()
}
