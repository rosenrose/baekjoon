use std::collections::HashMap;

const M: i64 = 1_000_000_007;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [n, m] = parse_int_vec(&buf)[..] else { return };
    let mut memo = HashMap::new();

    println!("{}", fibo_rem(get_gcd(n, m), &mut memo));
}

fn fibo_rem(n: i64, memo: &mut HashMap<i64, i64>) -> i64 {
    if n <= 1 {
        return n;
    }
    if n == 2 {
        return 1;
    }

    let i = n / 2;
    let j = if n % 2 == 0 { i } else { i + 1 };

    let mut get_or_insert = |num: i64| {
        if let Some(&ret) = memo.get(&num) {
            ret
        } else {
            let ret = fibo_rem(num, memo);
            memo.insert(num, ret);

            ret
        }
    };

    let (a, b) = (
        get_or_insert(i) * get_or_insert(j - 1),
        get_or_insert(i + 1) * get_or_insert(j),
    );

    (a % M + b % M) % M
}

fn get_gcd(mut a: i64, mut b: i64) -> i64 {
    loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    }
}

fn parse_int_vec(buf: &String) -> Vec<i64> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
