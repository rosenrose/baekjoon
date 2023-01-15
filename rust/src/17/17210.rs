use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<u32>);

    let n = input.next().unwrap();

    if n >= 6 {
        println!("Love is open door");
        return;
    }

    let mut door = input.next().unwrap();

    for _ in 0..n - 1 {
        door = if door == 0 { 1 } else { 0 };
        println!("{door}");
    }
}
