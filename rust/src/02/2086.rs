use std::collections::HashMap;

const M: i64 = 1_000_000_000;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [a, b] = parse_int_vec(&buf)[..] else { return };
    let mut cache = HashMap::new();

    println!(
        "{}",
        (fibo_rem(b + 2, &mut cache) - fibo_rem(a + 1, &mut cache) + M) % M
    );
}

fn fibo_rem(n: i64, cache: &mut HashMap<i64, i64>) -> i64 {
    if n <= 1 {
        return n;
    }
    if n == 2 {
        return 1;
    }

    let i = n / 2;
    let j = if n % 2 == 0 { i } else { i + 1 };

    let mut get_or_insert = |n: i64| match cache.get(&n) {
        Some(num) => *num,
        None => {
            let ret = fibo_rem(n, cache);
            cache.insert(n, ret);

            ret
        }
    };

    let (a, b) = (
        get_or_insert(i) * get_or_insert(j - 1),
        get_or_insert(i + 1) * get_or_insert(j),
    );

    (a % M + b % M) % M
}

fn parse_int_vec(buf: &String) -> Vec<i64> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
