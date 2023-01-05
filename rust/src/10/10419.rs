// use std::io::{stdin, Read};

// fn main() {
//     let mut buf = String::new();
//     stdin().read_to_string(&mut buf).unwrap();

//     let mut input = buf
//         .split_ascii_whitespace()
//         .map(|s| s.parse::<i32>().unwrap());

//     let (n, x) = (input.next().unwrap(), input.next().unwrap());
//     let cycle = input.map(|t| )
// }

use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf.lines().map(|s| s.parse::<i32>().unwrap());

    for d in input.skip(1) {
        let mut t = 0;
        let t = loop {
            if t * t + t > d {
                break t - 1;
            }

            t += 1;
        };

        println!("{t}");
    }
}
