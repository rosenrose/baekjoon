use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i64>);

    let (_, n) = (input.next(), input.next().unwrap());
    let cables: Vec<_> = input.collect();
    let max_length = *cables.iter().max().unwrap();

    println!("{}", binary_search(&cables, n, 1, max_length));
}

fn binary_search(cables: &Vec<i64>, n: i64, left: i64, right: i64) -> i64 {
    // println!("{left} {} {right}", (left + right) / 2);
    let is_ok = |num| cables.iter().map(|len| len / num).sum::<i64>() >= n;

    if is_ok(right) {
        return right;
    }
    if right - left == 1 {
        return left;
    }

    let mid = (left + right) / 2;

    if is_ok(mid) {
        binary_search(cables, n, mid, right)
    } else {
        binary_search(cables, n, left, mid)
    }
}
