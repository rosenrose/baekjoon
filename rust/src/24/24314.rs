use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<f64>);
    let [a1, a0, c, n0] = [(); 4].map(|_| input.next().unwrap());

    if c == a1 {
        println!("{}", u8::from(a0 >= 0.0));
        return;
    }
    if (c - a1).is_sign_positive() {
        println!("0");
        return;
    }

    let n = a0 / (c - a1);

    println!("{}", u8::from(n0 >= n));
}
