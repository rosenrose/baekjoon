use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i64>);

    let (_, m) = (input.next(), input.next().unwrap());
    let mut max_height = 0;

    let heights: Vec<_> = input
        .map(|height| {
            max_height = height.max(max_height);
            height
        })
        .collect();

    println!("{}", binary_search(&heights, m, 0, max_height - 1));
}

fn binary_search(heights: &Vec<i64>, m: i64, start: i64, end: i64) -> i64 {
    // println!("{start} {} {end}", (start + end) / 2);
    let is_ok = |num: i64| heights.iter().map(|h| (h - num).max(0)).sum::<i64>() >= m;

    if is_ok(end) {
        return end;
    }
    if end - start == 1 {
        return start;
    }

    let mid = (start + end) / 2;

    if is_ok(mid) {
        binary_search(heights, m, mid, end)
    } else {
        binary_search(heights, m, start, mid)
    }
}
