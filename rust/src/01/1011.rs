use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap());
    let mut output = String::new();

    for _ in 0..input.next().unwrap() {
        let (x, y) = (input.next().unwrap(), input.next().unwrap());
        let distance = y - x;

        let mut step = 0;
        let mut max_distance = 0;

        while max_distance < distance {
            step += 1;

            max_distance += if step % 2 == 0 {
                step / 2
            } else {
                (step + 1) / 2
            };
        }

        writeln!(output, "{step}").unwrap();
    }

    print!("{output}");
}

/*
1 - 1
2 - 1 1
3 - 1 2 1
4 - 1 2 2 1
5 - 1 2 3 2 1
6 - 1 2 3 3 2 1
7 - 1 2 3 4 3 2 1
8 - 1 2 3 4 4 3 2 1
*/
