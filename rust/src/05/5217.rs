use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
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
