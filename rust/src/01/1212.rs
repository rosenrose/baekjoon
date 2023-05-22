use std::fmt::Write;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut output = String::new();

    for (i, ch) in buf.trim().as_bytes().iter().enumerate() {
        let digit = ch - b'0';

        if i == 0 {
            write!(output, "{digit:b}")
        } else {
            write!(output, "{digit:03b}")
        }
        .unwrap();
    }

    print!("{output}");
}
