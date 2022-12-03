use std::fmt::Write;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut output = String::new();

    buf.trim().char_indices().for_each(|(i, c)| {
        let digit = c.to_digit(10).unwrap();

        if i == 0 {
            write!(output, "{digit:b}").unwrap();
        } else {
            write!(output, "{digit:03b}").unwrap();
        }
    });

    print!("{output}");
}
