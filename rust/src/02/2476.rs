use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let max_prize = (0..input.next().unwrap())
        .map(|_| match [(); 3].map(|_| input.next().unwrap()) {
            [a, b, c] if a == b && b == c => 10000 + a * 1000,
            [a, b, _] if a == b => 1000 + a * 100,
            [_, b, c] if b == c => 1000 + b * 100,
            [a, _, c] if c == a => 1000 + c * 100,
            nums => nums.iter().max().unwrap() * 100,
        })
        .max()
        .unwrap();

    println!("{max_prize}");
}
