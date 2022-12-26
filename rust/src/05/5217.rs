use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf.lines().map(|s| s.parse::<i32>().unwrap());

    for n in input.skip(1) {
        print!("Pairs for {n}: ");

        for (i, a) in (1..(n + 1) / 2).enumerate() {
            if i > 0 {
                print!(", ");
            }

            print!("{a} {}", n - a);
        }

        println!("");
    }
}
