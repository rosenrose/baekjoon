use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.lines();

    let n: usize = input.next().unwrap().parse().unwrap();
    let is_cute = input.filter(|&i| i == "1").count() > n / 2;

    println!("Junhee is {}!", if is_cute { "cute" } else { "not cute" });
}
