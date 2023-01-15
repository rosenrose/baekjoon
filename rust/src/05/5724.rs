use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<i32>);

    for n in input.take_while(|&n| n != 0) {
        let squares: i32 = (1..=n).map(|i| (n - i + 1).pow(2)).sum();

        println!("{squares}");
    }
}
