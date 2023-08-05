use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let [h, n] = [(); 2].map(|_| input.next().unwrap());

    println!("{}", h.matches(n).count());
}
