fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [n, p] = parse_int_vec(&buf)[..] else {
        return;
    };
    let mut nums = vec![n];
    let mut next = n;

    let pos = loop {
        next = (next * n) % p;

        if let Some(p) = nums.iter().position(|&num| num == next) {
            break p;
        }

        nums.push(next);
    };
    // println!("{nums:?}");
    println!("{}", &nums[pos..].len());
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
