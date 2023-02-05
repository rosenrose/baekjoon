use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<usize>);
    let mut output = String::new();

    let mut cache = vec![1, 1, 2];

    for i in 3..=30 {
        cache.push((0..i).map(|j| cache[j] * cache[i - 1 - j]).sum::<i64>());
    }
    // println!("{cache:?}");
    for n in input.take_while(|&n| n != 0) {
        writeln!(output, "{}", cache[n]).unwrap();
    }

    print!("{output}");
}
