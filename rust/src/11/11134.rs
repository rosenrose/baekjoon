use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<u32>);

    for [n, c] in (0..input.next().unwrap()).map(|_| [(); 2].map(|_| input.next().unwrap())) {
        println!("{}", n.div_ceil(c));
    }
}
