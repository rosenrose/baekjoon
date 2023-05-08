use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let (r, c, a, b) = (input(), input(), input(), input());

    for row in 0..r {
        for _ in 0..a {
            for col in 0..c {
                match (row % 2, col % 2) {
                    (0, 0) | (1, 1) => print!("{}", "X".repeat(b)),
                    (0, 1) | (1, 0) => print!("{}", ".".repeat(b)),
                    _ => unreachable!(),
                }
            }

            println!("");
        }
    }
}
