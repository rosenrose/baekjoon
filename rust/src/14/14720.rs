use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    input.next();

    let (_, count) = input.fold((-1, 0), |(milk, acc), next| match (milk, next) {
        (_, 0) if acc == 0 => (next, 1),
        (0, 1) | (1, 2) | (2, 0) => (next, acc + 1),
        _ => (milk, acc),
    });

    println!("{count}");
}
