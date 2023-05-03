fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [x, y] = parse_int_vec(&buf)[..] else { return };
    let z = (y * 100).checked_div(x);

    if z.is_none() || z.unwrap() >= 99 {
        println!("-1");
        return;
    }

    println!("{}", binary_search(x, y, z.unwrap()));
}

fn binary_search(x: i64, y: i64, z: i64) -> i64 {
    let is_ok = |diff: i64| (y + diff) * 100 / (x + diff) != z;
    let (mut lo, mut hi) = (1, x);
    let mut result = 0;

    while lo <= hi {
        let mid = lo + ((hi - lo) >> 1);

        if is_ok(mid) {
            result = mid;
            hi = mid - 1;
        } else {
            lo = mid + 1;
        }
    }

    result
}

fn parse_int_vec(buf: &str) -> Vec<i64> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
