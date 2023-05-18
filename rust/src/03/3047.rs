use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let mut abc: Vec<i32> = input.by_ref().take(3).flat_map(str::parse).collect();
    abc.sort();

    for ch in input.next().unwrap().as_bytes() {
        print!("{} ", abc[(ch - b'A') as usize]);
    }
}
