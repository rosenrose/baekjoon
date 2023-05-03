use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i64>);

    let (_, n) = (input.next(), input.next().unwrap());
    let cables: Vec<_> = input.collect();

    println!("{}", binary_search(&cables, n));
}

fn binary_search(cables: &[i64], n: i64) -> i64 {
    let is_ok = |num| cables.iter().map(|len| len / num).sum::<i64>() >= n;
    let (mut lo, mut hi) = (1, *cables.iter().max().unwrap());
    let mut result = 0;

    while lo <= hi {
        let mid = lo + ((hi - lo) >> 1);

        if is_ok(mid) {
            result = mid;
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }

    result
}
