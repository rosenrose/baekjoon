use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().skip(1).flat_map(str::parse::<i32>);

    let carlos = input.next().unwrap();

    println!(
        "{}",
        if input.all(|vote| carlos >= vote) {
            'S'
        } else {
            'N'
        }
    );
}
