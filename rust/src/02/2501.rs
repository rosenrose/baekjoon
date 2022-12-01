use std::collections::BTreeSet;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [n, k] = parse_int_vec(&buf)[..] {
        let divisors = (1..)
            .take_while(|i| i * i <= n)
            .fold(BTreeSet::new(), |mut acc, i| {
                if n % i == 0 {
                    acc.insert(i);
                    acc.insert(n / i);
                }

                acc
            });

        let divisors: Vec<_> = divisors.into_iter().collect();

        println!("{}", divisors.get(k as usize - 1).unwrap_or(&0));
    }
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
