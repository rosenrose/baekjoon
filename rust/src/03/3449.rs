use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let n: i32 = input.next().unwrap().parse().unwrap();

    for (a, b) in (0..n).map(|_| (input.next().unwrap(), input.next().unwrap())) {
        let dist = a.chars().zip(b.chars()).filter(|(a, b)| a != b).count();

        println!("Hamming distance is {dist}.");
    }
}
