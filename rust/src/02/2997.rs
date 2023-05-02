use std::cmp::Ordering;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut nums = parse_int_vec(&buf);
    nums.sort();

    let [a, b, c] = nums[..] else { return };

    match (b - a).cmp(&(c - b)) {
        Ordering::Equal => println!("{}", c + (c - b)),
        Ordering::Less => println!("{}", b + (b - a)),
        Ordering::Greater => println!("{}", a + (c - b)),
    }
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
