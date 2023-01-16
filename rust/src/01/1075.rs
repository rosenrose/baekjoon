use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let (n, f) = (input.next().unwrap(), input.next().unwrap());
    let n = n - n % 100;
    let mut num = (n / f) * f;

    while num < n {
        num += f;
    }

    println!("{:02}", num % 100);
}
