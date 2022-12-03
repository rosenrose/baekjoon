use std::fmt::Write;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut output = String::new();

    buf.trim().as_bytes().rchunks(3).rev().for_each(|chunk| {
        let digit: u8 = chunk
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &c)| if c as char == '1' { 1 << i } else { 0 })
            .sum();

        write!(output, "{digit}").unwrap();
    });

    print!("{output}");
}
