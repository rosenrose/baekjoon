use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let (n, k) = (input.next().unwrap(), input.next().unwrap());
    let mut nums = Vec::with_capacity(n as usize);

    for num in input {
        nums.push(num);
    }

    println!("{}", nums.select_nth_unstable(k as usize - 1).1);
}
