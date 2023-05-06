use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    for (i, (a, b, c)) in (1..).map(|i| (i, (input(), input(), input()))) {
        let (unknown, square) = match (a, b, c) {
            (0, 0, 0) => return,
            (-1, b, c) => ("a", c * c - b * b),
            (a, -1, c) => ("b", c * c - a * a),
            (a, b, -1) => ("c", a * a + b * b),
            _ => unreachable!(),
        };

        println!("Triangle #{i}");

        if square <= 0 {
            println!("Impossible.\n");
            continue;
        }

        println!("{unknown} = {:.3}\n", (square as f64).sqrt());
    }
}
