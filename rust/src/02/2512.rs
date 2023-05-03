use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i64>);

    let budget = input.next_back().unwrap();
    let (mut sum, mut max_req) = (0, 0);
    let requests: Vec<_> = input
        .skip(1)
        .map(|req| {
            sum += req;
            max_req = req.max(max_req);
            req
        })
        .collect();

    if sum <= budget {
        println!("{max_req}");
        return;
    }

    println!("{}", binary_search(&requests, budget, max_req));
}

fn binary_search(requests: &[i64], budget: i64, max_req: i64) -> i64 {
    let is_ok = |limit: i64| requests.iter().map(|&req| req.min(limit)).sum::<i64>() <= budget;
    let (mut lo, mut hi) = (budget / requests.len() as i64, max_req);
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
