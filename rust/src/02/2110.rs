use std::io;

const MAX: usize = 200_000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [n, c] = [(); 2].map(|_| input.next().unwrap() as usize);
    let mut routers = [0; MAX];

    for (i, num) in input.enumerate() {
        routers[i] = num;
    }

    routers[..n].sort_unstable();

    println!("{}", binary_search(&routers[..n], c));
}

fn binary_search(routers: &[i32], router_count: usize) -> i32 {
    let is_ok = |gap: i32| {
        let mut count = router_count - 1;
        let mut cursor = 0;

        for i in 1..routers.len() {
            if routers[i] - routers[cursor] < gap {
                continue;
            }

            count -= 1;
            cursor = i;

            if count == 0 {
                return true;
            }
        }

        false
    };
    let (mut lo, mut hi) = (1, routers.last().unwrap() - routers[0]);
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
