fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [n, m] = parse_int_vec(&buf)[..] else { return };
    let count_2 =
        get_multiple_count(n, 2) - get_multiple_count(n - m, 2) - get_multiple_count(m, 2);
    let count_5 =
        get_multiple_count(n, 5) - get_multiple_count(n - m, 5) - get_multiple_count(m, 5);

    println!("{}", count_2.min(count_5));
}

fn get_multiple_count(num: i64, divisor: i64) -> i64 {
    let (mut count, mut exp) = (0, divisor);

    while exp <= num {
        count += num / exp;
        exp *= divisor;
    }

    count
}

fn parse_int_vec(buf: &String) -> Vec<i64> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
