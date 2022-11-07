use std::cmp::Ordering;

fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let parse_int = |s: &str| s.parse::<i32>().unwrap();
    let n = parse_int(buf.trim());

    for _ in 0..n {
        let (y, k) = (0..9)
            .map(|_| {
                read_line(&mut buf);
                let mut nums = buf.split_whitespace().map(parse_int);

                (nums.next().unwrap(), nums.next().unwrap())
            })
            .reduce(|a, b| (a.0 + b.0, a.1 + b.1))
            .unwrap();

        match y.cmp(&k) {
            Ordering::Greater => println!("Yonsei"),
            Ordering::Less => println!("Korea"),
            Ordering::Equal => println!("Draw"),
        };
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
