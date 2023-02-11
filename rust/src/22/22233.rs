use std::collections::HashSet;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut output = String::new();

    let (n, _) = (input.next().unwrap().parse::<i32>().unwrap(), input.next());
    let mut memo: HashSet<_> = (0..n).map(|_| input.next().unwrap()).collect();

    for blog in input {
        for keyword in blog.split(',') {
            memo.remove(keyword);
        }

        writeln!(output, "{}", memo.len()).unwrap();
    }

    print!("{output}");
}
