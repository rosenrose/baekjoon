use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let (n, _) = (input.next().unwrap(), input.next());
    let max = (0..n).map(|_| input.next().unwrap()).max().unwrap() + input.max().unwrap();

    println!("{max}");
}
