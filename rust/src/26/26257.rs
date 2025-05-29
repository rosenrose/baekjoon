use std::fmt::Write;
use std::io;

const MAX: usize = 200_000 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let [_n, m, q] = [(); 3].map(|_| parse_int(input()));
    let mut pointers = [0; MAX];

    for (i, e) in (1..=m).map(|i| (i, parse_int(input()))) {
        pointers[i] = e;
    }

    for _ in 0..q {
        match input() {
            "assign" => {
                let [x, y] = [(); 2].map(|_| parse_int(input()));
                pointers[x] = pointers[y];
            }
            "swap" => pointers.swap(parse_int(input()), parse_int(input())),
            "reset" => pointers[parse_int(input())] = 0,
            _ => unreachable!(),
        }
    }
    // println!("{pointers:?}");
    let mut remains: Vec<_> = pointers[..=m].iter().filter(|&&p| p != 0).collect();
    remains.sort_unstable();
    remains.dedup();

    writeln!(output, "{}", remains.len()).unwrap();

    for object in remains {
        writeln!(output, "{object}").unwrap();
    }

    print!("{output}");
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
