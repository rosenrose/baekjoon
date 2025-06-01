use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [a, b] = [(); 2].map(|_| input.next().unwrap());
    let sequence = (1..).flat_map(|n| std::iter::repeat_n(n, n));

    println!("{}", sequence.skip(a - 1).take(b - a + 1).sum::<usize>());
}
