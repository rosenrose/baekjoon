use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();

    let mut infos: Vec<_> = (1..=input())
        .map(|i| (i, (input(), input(), input())))
        .collect();

    infos.sort_unstable_by(|(_, (s1, c1, t1)), (_, (s2, c2, t2))| {
        if s1 == s2 {
            (c1, t1).cmp(&(c2, t2))
        } else {
            s2.cmp(s1)
        }
    });

    println!("{}", infos[0].0);
}
