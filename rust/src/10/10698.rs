use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    for i in 1..=parse_int(input.next().unwrap()) {
        let [x, op, y, _, z] = [(); 5].map(|_| input.next().unwrap());
        let [x, y, z] = [x, y, z].map(parse_int);

        println!(
            "Case {i}: {}",
            match op {
                "+" if x + y == z => "YES",
                "-" if x - y == z => "YES",
                _ => "NO",
            }
        );
    }
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
