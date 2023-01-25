use std::collections::HashMap;

const M: i64 = 1_000_000_007;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i64 = buf.trim().parse().unwrap();
    let mut cache = HashMap::new();

    println!(
        "{}",
        ((fibo_rem(n, &mut cache) % M) * (fibo_rem(n + 1, &mut cache) % M)) % M
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

    let mut get_or_insert = |n: i64| {
        if let Some(num) = cache.get(&n) {
            *num
        } else {
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
