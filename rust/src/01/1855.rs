use std::io;

const MAX: usize = 200;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let width: usize = input.next().unwrap().parse().unwrap();
    let encrypted = input.next().unwrap().as_bytes();
    let height = encrypted.len() / width;
    let mut table = [(); MAX].map(|_| Vec::new());

    for (r, chunk) in encrypted.chunks(width).enumerate() {
        let mut row = chunk.to_vec();

        if r % 2 == 1 {
            row.reverse();
        }

        table[r] = row;
    }

    for c in 0..width {
        for r in 0..height {
            print!("{}", table[r][c] as char);
        }
    }
}
