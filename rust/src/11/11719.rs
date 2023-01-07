use std::io;

fn main() {
    print!("{}", io::read_to_string(io::stdin()).unwrap());
}

/* loop {
    buf.clear();

    match std::io::stdin().read_line(&mut buf).unwrap() {
        0 => return,
        _ => print!("{buf}"),
    };
} */
