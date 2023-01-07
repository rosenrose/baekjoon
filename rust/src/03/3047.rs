use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let mut abc: Vec<i32> = (0..3)
        .map(|_| input.next().unwrap().parse().unwrap())
        .collect();
    abc.sort();

    for ch in input.next().unwrap().as_bytes() {
        print!("{} ", abc[(ch - 'A' as u8) as usize]);
    }
}
