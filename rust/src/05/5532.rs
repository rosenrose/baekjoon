use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(str::parse::<i32>);

    let [l, a, b, c, d] = [(); 5].map(|_| input.next().unwrap());
    let lang_count = (a as f64 / c as f64).ceil() as i32;
    let math_count = (b as f64 / d as f64).ceil() as i32;

    println!("{}", l - lang_count.max(math_count));
}
