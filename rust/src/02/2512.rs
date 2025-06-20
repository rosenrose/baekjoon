use std::io;

const MAX: usize = 10000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let budget = input.next_back().unwrap();
    let (mut sum, mut max_req) = (0, 0);
    let mut requests = [0; MAX];

    for (i, req) in input.enumerate() {
        sum += req;
        max_req = req.max(max_req);
        requests[i] = req;
    }

    if sum <= budget {
        println!("{max_req}");
        return;
    }

    println!("{}", binary_search(&requests[..n], budget, max_req));
}

fn binary_search(requests: &[usize], budget: usize, max_req: usize) -> usize {
    let is_ok = |limit: usize| requests.iter().map(|&req| req.min(limit)).sum::<usize>() <= budget;
    let (mut lo, mut hi) = (budget / requests.len(), max_req);
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
