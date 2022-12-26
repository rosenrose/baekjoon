use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf.lines().map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    for (i, n0) in input.take_while(|&n| n != 0).enumerate() {
        let n1 = 3 * n0;
        let n2 = if n1 % 2 == 0 { n1 / 2 } else { (n1 + 1) / 2 };
        let n3 = 3 * n2;

        writeln!(
            output,
            "{}. {} {}",
            i + 1,
            if n1 % 2 == 0 { "even" } else { "odd" },
            n3 / 9
        )
        .unwrap();
    }

    print!("{output}");
}
