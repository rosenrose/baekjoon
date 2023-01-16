use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    (1..)
        .map_while(|i| {
            let (a, b, c) = (input(), input(), input());
            ((a, b, c) != (0, 0, 0)).then_some((i, (a, b, c)))
        })
        .for_each(|(i, (a, b, c))| {
            let (name, square) = match (a, b, c) {
                (-1, b, c) => ("a", c * c - b * b),
                (a, -1, c) => ("b", c * c - a * a),
                (a, b, -1) => ("c", a * a + b * b),
                _ => return,
            };

            println!("Triangle #{i}");

            if square <= 0 {
                println!("Impossible.\n");
                return;
            }

            println!("{name} = {:.3}\n", (square as f64).sqrt());
        });
}
