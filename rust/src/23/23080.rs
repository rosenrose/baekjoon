use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let k: usize = input.next().unwrap().parse().unwrap();
    let s = input.next().unwrap();

    println!(
        "{}",
        s.char_indices()
            .filter_map(|(i, c)| (i % k == 0).then(|| c))
            .collect::<String>()
    );
}
