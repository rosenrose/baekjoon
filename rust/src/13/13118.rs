use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let points: Vec<_> = input.by_ref().take(4).collect();
    let x = input.next().unwrap();

    println!(
        "{}",
        if let Some(i) = points.iter().position(|&p| p == x) {
            i + 1
        } else {
            0
        }
    );
}
