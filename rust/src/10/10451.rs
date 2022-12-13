use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap());
    let mut output = String::new();

    for _ in 0..input.next().unwrap() {
        let n = input.next().unwrap();
        let shuffle: Vec<_> = (0..n).map(|_| input.next().unwrap() - 1).collect();

        let mut checked = vec![false; n];
        let mut count = 0;

        for i in 0..checked.len() {
            if checked[i] {
                continue;
            }

            let mut next = shuffle[i];
            let mut nodes = vec![i];

            while next != i {
                nodes.push(next);
                next = shuffle[next];
            }

            for j in nodes {
                checked[j] = true;
            }

            count += 1;
        }

        writeln!(output, "{count}").unwrap();
    }

    print!("{output}");
}
