use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let k: usize = input.next().unwrap().parse().unwrap();
    let table: Vec<_> = input
        .next()
        .unwrap()
        .as_bytes()
        .chunks(k)
        .enumerate()
        .map(|(i, chunk)| {
            let mut chunk = chunk.to_vec();

            if i % 2 == 1 {
                chunk.reverse();
            }

            chunk
        })
        .collect();

    for col in 0..k {
        for row in &table {
            print!("{}", row[col] as char);
        }
    }
}
