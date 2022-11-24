fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let a = parse_int_vec(&buf);
    read_line(&mut buf);

    let b = parse_int_vec(&buf);

    if let [a1, b1, c1, d, a2, b2, c2, _] = [a, b].concat()[..] {
        let a1a2 = a1 * a2;
        let (a2b1, a1b2, a2c1, a1c2) = (a2 * b1, a1 * b2, a2 * c1, a1 * c2);
        let (b1b2, c1c2d, b2c1, b1c2) = (b1 * b2, c1 * c2 * d, b2 * c1, b1 * c2);

        println!(
            "{}\n{}\n{}\n{}",
            vec_join(&normalize(a1a2, a2b1 + a1b2, a2c1 + a1c2, d), " "),
            vec_join(&normalize(a1a2, a2b1 - a1b2, a2c1 - a1c2, d), " "),
            vec_join(&normalize(a1a2, b1b2 + c1c2d, b2c1 + b1c2, d), " "),
            vec_join(
                &normalize(
                    a1 * (b2 * b2 - c2 * c2 * d),
                    a2 * (b1b2 - c1c2d),
                    a2 * (b2c1 - b1c2),
                    d
                ),
                " "
            )
        );
    }
}

fn normalize(mut a: i64, mut b: i64, mut c: i64, mut d: i64) -> Vec<i64> {
    if c == 0 {
        d = 0;
    }
    if d == 0 {
        c = 0;
    }

    let gcd = get_gcd([a, b, c].into_iter().filter(|&i| i != 0));
    (a, b, c) = (a / gcd, b / gcd, c / gcd);

    if a < 0 {
        a *= -1;
        b *= -1;
        c *= -1;
    }

    vec![a, b, c, d]
}

fn get_gcd<I>(nums: I) -> i64
where
    I: Iterator<Item = i64>,
{
    nums.reduce(|mut a, mut b| loop {
        (a, b) = (b, a % b);

        if b == 0 {
            return a;
        }
    })
    .unwrap()
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i64> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn vec_join<T>(vec: &Vec<T>, seperator: &str) -> String
where
    T: ToString,
{
    vec.iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>()
        .join(seperator)
}
