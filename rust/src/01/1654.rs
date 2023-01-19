use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i64>);

    let (_, n) = (input.next(), input.next().unwrap());
    let cables: Vec<_> = input.collect();
    let max_length = *cables.iter().max().unwrap();

    println!("{}", binary_search(&cables, n, 1, max_length));
}

fn binary_search(cables: &Vec<i64>, n: i64, start: i64, end: i64) -> i64 {
    // println!("{start} {} {end}", (start + end) / 2);
    let is_ok = |num| cables.iter().map(|len| len / num).sum::<i64>() >= n;

    if is_ok(end) {
        return end;
    }
    if end - start == 1 {
        return start;
    }

    let mid = (start + end) / 2;

    if is_ok(mid) {
        binary_search(cables, n, mid, end)
    } else {
        binary_search(cables, n, start, mid)
    }
}
