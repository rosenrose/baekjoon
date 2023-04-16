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

    let mut sum: i32 = students[1..=2]
        .iter()
        .map(|row| row.iter().sum::<i32>())
        .sum();
    let mut count = (sum as f64 / k).ceil() as i32;

    for gender in 0..=1 {
        sum = (3..=4).map(|grade| students[grade][gender]).sum();
        count += (sum as f64 / k).ceil() as i32;

        sum = (5..=6).map(|grade| students[grade][gender]).sum();
        count += (sum as f64 / k).ceil() as i32;
    }

    println!("{count}");
}
