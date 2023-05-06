use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut output = String::new();

    let mut encrypted = input.next().unwrap().as_bytes().chunks(2);

    for type_ in input.skip(1) {
        let bytes: Vec<_> = encrypted
            .by_ref()
            .take(match type_ {
                "char" => 1,
                "int" => 4,
                "long_long" => 8,
                _ => unreachable!(),
            })
            .flatten()
            .copied()
            .collect();
        let chunk = String::from_utf8(bytes).unwrap();
        let decrypted = u64::from_str_radix(&chunk, 16).unwrap();

        write!(output, "{decrypted} ").unwrap();
    }

    print!("{output}");
}
