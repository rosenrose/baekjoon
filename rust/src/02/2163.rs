use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [n, m] = [(); 2].map(|_| input.next().unwrap());
    let horizontal_count = n - 1;
    let vertical_count = (m - 1) * n;

    println!("{}", horizontal_count + vertical_count);
}
