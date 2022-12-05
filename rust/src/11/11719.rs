use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    print!("{buf}");

    /* loop {
        buf.clear();

        match std::io::stdin().read_line(&mut buf).unwrap() {
            0 => return,
            _ => print!("{buf}"),
        };
    } */
}
