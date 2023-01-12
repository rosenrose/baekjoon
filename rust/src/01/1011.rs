use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap());
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    for (x, y) in (0..input()).map(|_| (input(), input())) {
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
