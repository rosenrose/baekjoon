use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<f32>);

    while let (Some(h), Some(p)) = (input.next(), input.next()) {
        println!("{:.2}", h / p);
    }
}
