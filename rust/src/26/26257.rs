use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (_, m, q) = (input(), parse_int(input()), parse_int(input()));
    let mut pointers = vec![0; m + 1];

    for (i, e) in (1..=m).map(|i| (i, parse_int(input()))) {
        pointers[i] = e;
    }

    for _ in 0..q {
        match input() {
            "assign" => {
                let (x, y) = (parse_int(input()), parse_int(input()));
                pointers[x] = pointers[y];
            }
            "swap" => pointers.swap(parse_int(input()), parse_int(input())),
            "reset" => pointers[parse_int(input())] = 0,
            _ => (),
        }
    }
    // println!("{pointers:?}");
    let mut remains: Vec<_> = pointers.iter().filter(|&&p| p != 0).collect();

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
