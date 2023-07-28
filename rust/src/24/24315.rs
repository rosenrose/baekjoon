use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<f64>);

    let args = [(); 5].map(|_| input.next().unwrap());

    println!("{}", big_theta(args) as u8);
}

fn big_theta([a1, a0, c1, c2, n0]: [f64; 5]) -> bool {
    // 0 <= (a1-c1)*n + a0 && (a1-c2)*n + a0 <= 0
    big_omega([a1, a0, c1, n0]) && big_o([a1, a0, c2, n0])
}

fn big_omega([a1, a0, c, n0]: [f64; 4]) -> bool {
    // 0 <= (a1-c)*n + a0
    if a1 == c {
        return a0 >= 0.0;
    }
    if (a1 - c).is_sign_negative() {
        return false;
    }

    n0 >= -a0 / (a1 - c)
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
