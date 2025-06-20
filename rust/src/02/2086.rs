use std::collections::HashMap;
use std::io;

const M: i64 = 1_000_000_000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i64>);

    let [a, b] = [(); 2].map(|_| input.next().unwrap());
    let mut memo = HashMap::new();

    println!(
        "{}",
        (fibo_rem(b + 2, &mut memo) - fibo_rem(a + 1, &mut memo)).rem_euclid(M)
    );
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
