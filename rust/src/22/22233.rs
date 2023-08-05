use std::collections::HashSet;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut output = String::new();

    let [n, _] = [(); 2].map(|_| input.next().unwrap().parse::<usize>().unwrap());
    let mut memo: HashSet<_> = input.by_ref().take(n).collect();

    for blog in input {
        for keyword in blog.split(',') {
            memo.remove(keyword);
        }

        writeln!(output, "{}", memo.len()).unwrap();
    }

    print!("{output}");
}
