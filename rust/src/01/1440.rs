use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.trim().split(':').flat_map(str::parse::<i32>);

    let [a, b, c] = [(); 3].map(|_| input.next().unwrap());
    let mut count = 0;

    if matches!((a, b, c), (1..=12, 0..=59, 0..=59)) {
        count += 2;
    }
    if matches!((a, b, c), (0..=59, 1..=12, 0..=59)) {
        count += 2;
    }
    if matches!((a, b, c), (0..=59, 0..=59, 1..=12)) {
        count += 2;
    }

    println!("{count}");
}
