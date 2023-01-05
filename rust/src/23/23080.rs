use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

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
