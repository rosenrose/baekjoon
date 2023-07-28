use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<u32>);

    let n = input.next().unwrap();

    if n >= 6 {
        println!("Love is open door");
        return;
    }

    let door = input.next().unwrap();

    for i in 0..n - 1 {
        println!("{}", (i & 1 == door) as u8);
    }
}
