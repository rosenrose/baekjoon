use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [n, _] = [(); 2].map(|_| input.next().unwrap());
    let max = input.by_ref().take(n).max().unwrap() + input.max().unwrap();

    println!("{max}");
}
