use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let mut students = Vec::new();

    for (i, order) in input.skip(1).enumerate() {
        students.insert(order, i + 1);
    }

    for s in students.iter().rev() {
        print!("{s} ");
    }
}
