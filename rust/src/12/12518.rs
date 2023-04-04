use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();

    for (i, name) in buf.lines().enumerate().skip(1) {
        let ruler = match name.to_lowercase() {
            name if name.ends_with(['a', 'e', 'i', 'o', 'u']) => "a queen",
            name if name.ends_with('y') => "nobody",
            _ => "a king",
        };

        writeln!(output, "Case #{i}: {name} is ruled by {ruler}.").unwrap();
    }

    print!("{output}");
}
