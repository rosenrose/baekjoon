use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [x, n] = [(); 2].map(|_| input.next().unwrap());
    let sum: i32 = (0..n)
        .map(|_| input.by_ref().take(2).product::<i32>())
        .sum();

    println!("{}", if sum == x { "Yes" } else { "No" });
}
