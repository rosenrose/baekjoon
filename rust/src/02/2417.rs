fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: u64 = buf.trim().parse().unwrap();
    let sqrt = (n as f64).sqrt() as u64;

    if sqrt * sqrt >= n {
        println!("{sqrt}");
        return;
    }

    if (sqrt + 1) * (sqrt + 1) >= n {
        println!("{}", sqrt + 1);
    }
    // println!("{}", binary_search(n));
}

fn binary_search(num: u64) -> i64 {
    let is_ok = |sqrt: u64| sqrt * sqrt >= num;
    let (mut lo, mut hi) = (0, 1 << 32);
    let mut result = 0;

    while lo <= hi {
        let mid = lo + ((hi - lo) >> 1);

        if is_ok(mid as u64) {
            result = mid;
            hi = mid - 1;
        } else {
            lo = mid + 1;
        }
    }

    result
}
