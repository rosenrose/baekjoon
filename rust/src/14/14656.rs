use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    println!(
        "{}",
        input
            .enumerate()
            .skip(1)
            .filter(|&(i, num)| i != num)
            .count()
    );
}
