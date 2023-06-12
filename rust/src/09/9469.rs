use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<f64>);

    for _ in 0..input.next().unwrap() as i32 {
        let [n, d, a, b, f] = [(); 5].map(|_| input.next().unwrap());

        println!("{n} {:.2}", d / (a + b) * f);
    }
}
