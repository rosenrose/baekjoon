use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<i32>);

    let sum: i32 = [9, 7, 8, 0, 9, 2, 1, 4, 1, 8]
        .into_iter()
        .chain(input)
        .enumerate()
        .map(|(i, num)| num * if i % 2 == 0 { 1 } else { 3 })
        .sum();

    println!("The 1-3-sum is {sum}");
}
