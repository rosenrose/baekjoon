use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut count = 0;

    for (i, row) in buf.lines().enumerate() {
        for (j, c) in row.char_indices() {
            match (i % 2, j % 2, c) {
                (0, 0, 'F') | (1, 1, 'F') => count += 1,
                _ => (),
            };
        }
    }

    println!("{count}");
}
