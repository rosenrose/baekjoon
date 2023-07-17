use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut output = String::new();

    for _ in 0..input.next().unwrap() {
        let n = input.next().unwrap();
        let shuffle: Vec<_> = input.by_ref().take(n).map(|num| num - 1).collect();

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
