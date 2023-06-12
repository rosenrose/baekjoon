use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [n, _, _, d, r, _] = [(); 6].map(|_| input.next().unwrap());

    println!(
        "{}",
        match ((n % 2, d), r) {
            ((0, 0) | (1, 1), _) => "NO...",
            ((0, 1) | (1, 0), r) if r < n => "NO...",
            _ => "YES!",
        }
    );
}
