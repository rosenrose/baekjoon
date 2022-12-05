use std::collections::VecDeque;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap());

    for _ in 0..input.next().unwrap() {
        let (n, m) = (input.next().unwrap(), input.next().unwrap());

        let mut docs: VecDeque<_> = (0..n).map(|i| (input.next().unwrap(), i)).collect();

        if docs.len() == 1 {
            println!("1");
            continue;
        }

        let target = docs[m];
        let mut order = 1;

        loop {
            let (mut first_importance, _) = docs[0];

            while docs.iter().skip(1).any(|doc| doc.0 > first_importance) {
                let temp = docs.pop_front().unwrap();
                docs.push_back(temp);

                (first_importance, _) = docs[0];
            }

            if docs.pop_front().unwrap() == target {
                println!("{order}");
                break;
            }

            order += 1;
        }
    }
}
