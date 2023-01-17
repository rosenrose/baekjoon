use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let students = input
        .skip(1)
        .enumerate()
        .fold(Vec::new(), |mut acc, (i, order)| {
            acc.insert(order, i + 1);
            acc
        });

    for s in students.iter().rev() {
        print!("{s} ");
    }
}
