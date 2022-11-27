use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut a = buf
        .lines()
        .next_back()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let mut lis = vec![a.next().unwrap()];

    for num in a {
        if num > *lis.last().unwrap() {
            lis.push(num);
            continue;
        }

        let pos = match lis.binary_search(&num) {
            Ok(i) => i,
            Err(i) => i,
        };

        lis[pos] = num;
    }

    writeln!(stdout, "{}", lis.len()).unwrap();
}
