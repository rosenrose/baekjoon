use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<f64>);

    let [d, h, w] = [(); 3].map(|_| input.next().unwrap());
    let multiple = d / h.hypot(w);

    println!("{} {}", (h * multiple).floor(), (w * multiple).floor());
}
