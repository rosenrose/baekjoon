use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    let n: usize = input().parse().unwrap();
    let a: Vec<Vec<_>> = (0..n)
        .map(|_| (0..n).map(|_| input() == "1").collect())
        .collect();
    let b: Vec<Vec<_>> = (0..n)
        .map(|_| (0..n).map(|_| input() == "1").collect())
        .collect();

    let count: i32 = a
        .iter()
        .flat_map(|a_row| {
            (0..n).map(|i| {
                a_row
                    .iter()
                    .enumerate()
                    .map(|(j, a_num)| a_num & b[j][i])
                    .any(|and| and) as i32
            })
        })
        .sum();

    println!("{count}");
}
