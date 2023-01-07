use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let n: usize = input.next().unwrap().parse().unwrap();
    let is_cute = input.filter(|&i| i == "1").count() > n / 2;

    println!("Junhee is {}!", if is_cute { "cute" } else { "not cute" });
}
