use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<f64>);

    let [_, w, h] = [(); 3].map(|_| input.next().unwrap());

    for len in input {
        println!("{}", if len <= w.hypot(h) { "DA" } else { "NE" });
    }
}
