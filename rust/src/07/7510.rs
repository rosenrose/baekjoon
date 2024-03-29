use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i64>);
    let mut output = String::new();

    for (i, [a, b, c]) in
        (1..=input.next().unwrap()).map(|i| (i, [(); 3].map(|_| input.next().unwrap())))
    {
        let is_right = match a.max(b).max(c) {
            longest if longest == a => a * a == b * b + c * c,
            longest if longest == b => b * b == a * a + c * c,
            longest if longest == c => c * c == a * a + b * b,
            _ => unreachable!(),
        };

        writeln!(output, "Scenario #{i}:").unwrap();
        writeln!(output, "{}\n", if is_right { "yes" } else { "no" }).unwrap();
    }

    print!("{output}");
}
