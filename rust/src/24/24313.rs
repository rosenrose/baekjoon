use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<f32>);
    let [a1, a0, c, n0] = input.collect::<Vec<_>>()[..] else { return };

    if c == a1 {
        println!("{}", if a0 <= 0.0 { 1 } else { 0 });
        return;
    }
    if (c - a1).is_sign_negative() {
        println!("0");
        return;
    }

    let n = a0 / (c - a1);

    println!("{}", if n0 >= n { 1 } else { 0 });
}
