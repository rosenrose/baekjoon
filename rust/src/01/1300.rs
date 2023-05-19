use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(str::parse::<i64>);

    let (n, k) = (input.next().unwrap(), input.next().unwrap());

    println!("{}", binary_search(n, k));
}

fn binary_search(n: i64, k: i64) -> i64 {
    let get_less_count = |num: i64| (1..=num.min(n)).map(|i| num.min(i * n) / i).sum::<i64>();
    // for i in 1..=n * n {
    //     println!("{i}: {}", get_less_count(i));
    // }
    let (mut lo, mut hi) = (1, k);
    let mut result = 0;

    while lo <= hi {
        let mid = lo + ((hi - lo) >> 1);

        if get_less_count(mid) < k {
            lo = mid + 1;
        } else {
            result = mid;
            hi = mid - 1;
        }
    }

    result
}
