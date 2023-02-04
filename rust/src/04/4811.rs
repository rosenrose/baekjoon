use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<usize>);
    let mut output = String::new();

    let mut cache = vec![1, 1, 2];

    for i in 3..=30 {
        let mut next = (0..i / 2).map(|j| cache[j] * cache[i - 1 - j]).sum::<i64>() * 2;

        if i % 2 == 1 {
            next += cache[i / 2] * cache[i / 2];
        }

        cache.push(next);
    }
    // println!("{cache:?}");
    for n in input.take_while(|&n| n != 0) {
        writeln!(output, "{}", cache[n]).unwrap();
    }

    print!("{output}");
}
