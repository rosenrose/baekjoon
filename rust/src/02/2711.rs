use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();

    for _ in 0..parse_int(input.next().unwrap()) {
        let index = parse_int(input.next().unwrap()) - 1;
        let typo: String = input
            .next()
            .unwrap()
            .char_indices()
            .filter_map(|(i, c)| (i != index).then(|| c))
            .collect();

        println!("{typo}");
    }
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
