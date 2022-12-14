use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
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
