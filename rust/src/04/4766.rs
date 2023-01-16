use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let temperatures: Vec<f32> = buf
        .lines()
        .take_while(|&s| s != "999")
        .flat_map(str::parse)
        .collect();

    for i in 1..temperatures.len() {
        println!("{:.2}", temperatures[i] - temperatures[i - 1]);
    }
}
