use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();

    let regex = Regex::new("^(100+1+|01)+$", false);

    for input in buf.lines().skip(1) {
        writeln!(
            output,
            "{}",
            if regex.find(input).is_some() {
                "YES"
            } else {
                "NO"
            }
        )
        .unwrap();
    }

    print!("{output}");
}
