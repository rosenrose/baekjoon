use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let (n, k) = (input(), input() as f64);
    let mut students = [[0, 0]; 7];

    for (gender, grade) in (0..n).map(|_| (input(), input())) {
        students[grade][gender] += 1;
    }

    let count: i32 = students[1..]
        .iter()
        .map(|&[f, m]| (f as f64 / k).ceil() as i32 + (m as f64 / k).ceil() as i32)
        .sum();

    println!("{count}");
}
