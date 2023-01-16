use std::collections::VecDeque;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    for _ in 0..input() {
        let (n, m) = (input(), input() as usize);
        let mut docs: VecDeque<_> = (0..n).map(|i| (i, input())).collect();

        let target = docs[m];
        let mut order = 1;

        loop {
            while let Some(pos) = docs.iter().position(|doc| doc.1 > docs[0].1) {
                docs.rotate_left(pos);
            }

            if docs.pop_front().unwrap() == target {
                println!("{order}");
                break;
            }

            order += 1;
        }
    }
}
