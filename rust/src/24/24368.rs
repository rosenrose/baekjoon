use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<f64>);

    let args = [(); 5].map(|_| input.next().unwrap());

    println!("{}", u8::from(big_o(args)));
}

fn big_o([a2, a1, a0, c, n0]: [f64; 5]) -> bool {
    // (a2-c)*n^2 + a1*n + a0 <= 0
    let (a_, b_, c_) = (a2 - c, a1, a0);

    if is_zero(a_) {
        // a1*n + a0 <= 0
        if is_zero(a1) {
            return a0 <= 0.0;
        }
        if a1.is_sign_positive() {
            return false;
        }

        return n0 >= -a0 / a1;
    }
    if a_.is_sign_positive() {
        return false;
    }

    let discriminant = (b_ * b_ - 4.0 * a_ * c_).sqrt();

    if discriminant.is_nan() || is_zero(discriminant) {
        return true;
    }

    let solution_one = (-b_ - discriminant) / (2.0 * a_);
    let solution_two = (-b_ + discriminant) / (2.0 * a_);

    n0 >= solution_one.max(solution_two)
}

fn is_zero(num: f64) -> bool {
    (num - 0.0).abs() < 1e-10
}
