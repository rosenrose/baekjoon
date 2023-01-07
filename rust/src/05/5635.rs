use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    let n = parse_int(input());

    let infos: Vec<_> = (0..n)
        .map(|_| {
            let name = input();
            let (d, m, y) = (parse_int(input()), parse_int(input()), parse_int(input()));

            (name, (y, m, d))
        })
        .collect();

    let (youngest, _) = infos.iter().max_by_key(|(_, birth)| birth).unwrap();
    let (oldest, _) = infos.iter().min_by_key(|(_, birth)| birth).unwrap();

    println!("{youngest}\n{oldest}");
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
