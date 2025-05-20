use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [n, p] = [(); 2].map(|_| input.next().unwrap());
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
