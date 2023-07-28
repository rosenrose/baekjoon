use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let [s, p] = [(); 2].map(|_| input.next().unwrap());

    println!("{}", s.contains(p) as u8);
}
