fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [n, k] = parse_int_vec(&buf)[..] {
        let mut divisors = get_divisors(n);

        divisors.sort();

        println!("{}", divisors.get(k as usize - 1).unwrap_or(&0));
    }
}

fn get_divisors(num: i32) -> Vec<i32> {
    (1..)
        .take_while(|i| i * i <= num)
        .filter(|i| num % i == 0)
        .map(|i| {
            let mut divisor = vec![i, num / i];
            divisor.dedup();

            divisor
        })
        .flatten()
        .collect()
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
