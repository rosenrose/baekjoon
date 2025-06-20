use std::fmt::Write;
use std::io;

const MAX: usize = 100_000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let n = input.next().unwrap() as usize;
    let mut a = [0; MAX];
    let mut b = [0; MAX];

    for (i, num) in input.by_ref().take(n).enumerate() {
        a[i] = num;
    }

    for (i, num) in input.by_ref().take(n).enumerate() {
        b[i] = num;
    }

    let m = input.next().unwrap() as usize;

    for num in b[..n]
        .iter()
        .enumerate()
        .rev()
        .filter_map(|(i, &num)| (a[i] == 0).then_some(num))
        .chain(input)
        .take(m)
    {
        write!(output, "{num} ").unwrap();
    }

    print!("{output}");
}
