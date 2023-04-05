use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().map(|input| input.matches('S').count());

    let product = input.next().unwrap() * input.next().unwrap();
    println!("{}0{}", "S(".repeat(product), ")".repeat(product));
}
