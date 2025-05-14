use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    let n: i32 = input().parse().unwrap();

    for (a, b) in (0..n).map(|_| (input(), input())) {
        let distances =
            a.as_bytes()
                .iter()
                .zip(b.as_bytes())
                .map(|(x, y)| if x <= y { y - x } else { 26 - (x - y) });

        print!("Distances: ");

        for dist in distances {
            print!("{dist} ");
        }

        println!("");
    }
}
