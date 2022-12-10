use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let n = parse_int(input.next().unwrap());

    let infos: Vec<_> = (0..n)
        .map(|_| {
            let name = input.next().unwrap();
            let (d, m, y) = (
                parse_int(input.next().unwrap()),
                parse_int(input.next().unwrap()),
                parse_int(input.next().unwrap()),
            );

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
