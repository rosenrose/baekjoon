use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let [_, init, k] = [(); 3].map(|_| input());
    let x = (0..k).fold(init, |x, _| match (input(), input()) {
        (a, b) if a == x => b,
        (a, b) if b == x => a,
        _ => x,
    });

    println!("{x}");
}
