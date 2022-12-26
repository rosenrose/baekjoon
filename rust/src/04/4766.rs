use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let temperatures: Vec<_> = buf
        .lines()
        .take_while(|&s| s != "999")
        .map(|s| s.parse::<f32>().unwrap())
        .collect();

    for i in 1..temperatures.len() {
        println!("{:.2}", temperatures[i] - temperatures[i - 1]);
    }
}
