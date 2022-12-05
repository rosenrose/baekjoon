use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();

    let n: i32 = input.next().unwrap().parse().unwrap();
    let (before, after) = (input.next().unwrap(), input.next().unwrap());

    for (x, y) in before.chars().zip(after.chars()) {
        match (n % 2, x == y) {
            (0, false) | (1, true) => {
                println!("Deletion failed");
                return;
            }
            _ => (),
        };
    }

    println!("Deletion succeeded");
}
