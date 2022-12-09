use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let n: i32 = input.next().unwrap().parse().unwrap();

    for _ in 0..n {
        let (a, b) = (input.next().unwrap(), input.next().unwrap());

        let distances =
            a.as_bytes().iter().zip(b.as_bytes()).map(
                |(x, y)| {
                    if x <= y {
                        y - x
                    } else {
                        26 - (x - y)
                    }
                },
            );

        print!("Distances: ");
        for dist in distances {
            print!("{dist} ");
        }

        println!("");
    }
}
