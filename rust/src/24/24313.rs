use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<f64>);

    let args = [(); 4].map(|_| input.next().unwrap());

    println!("{}", big_o(args) as u8);
}

fn big_o([a1, a0, c, n0]: [f64; 4]) -> bool {
    // (a1-c)*n + a0 <= 0
    if a1 == c {
        return a0 <= 0.0;
    }
    if (a1 - c).is_sign_positive() {
        return false;
    }

    n0 >= -a0 / (a1 - c)
}
