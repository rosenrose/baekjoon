use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    const NO: &str = "NO";

    'outer: for input in buf.lines().skip(1) {
        let mut count = 0;

        for c in input.chars() {
            match c {
                '(' => count += 1,
                ')' => {
                    if count == 0 {
                        println!("{NO}");
                        continue 'outer;
                    }

                    count -= 1;
                }
                _ => (),
            };
        }

        println!("{}", if count == 0 { "YES" } else { NO });
    }
}
