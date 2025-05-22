use std::fmt::Write;
use std::io;

const MAX: usize = 1000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut output = String::new();

    for _ in 0..input.next().unwrap() {
        let n = input.next().unwrap();
        let mut shuffle = [0; MAX];

        for (i, num) in input.by_ref().take(n).enumerate() {
            shuffle[i] = num - 1;
        }

        let mut checked = [false; MAX];
        let mut count = 0;

        for i in 0..n {
            if checked[i] {
                continue;
            }

            let mut next = shuffle[i];
            let mut nodes = Vec::with_capacity(MAX);
            nodes.push(i);

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
