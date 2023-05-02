use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let (_, m) = (input.next(), input.next().unwrap() as i64);
    let heights: Vec<_> = input.collect();

    println!("{}", binary_search(&heights, m));
}

fn binary_search(heights: &Vec<i32>, m: i64) -> i32 {
    let is_ok = |num: i32| {
        heights
            .iter()
            .map(|h| ((h - num) as i64).max(0))
            .sum::<i64>()
            >= m
    };
    let (mut lo, mut hi) = (0, heights.iter().max().unwrap() - 1);
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
