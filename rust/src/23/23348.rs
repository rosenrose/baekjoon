use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let [a, b, c] = [(); 3].map(|_| input());
    let n = input();

    let max_score = (0..n)
        .map(|_| {
            (0..3)
                .map(|_| a * input() + b * input() + c * input())
                .sum::<i32>()
        })
        .max()
        .unwrap();

    println!("{max_score}")
}
