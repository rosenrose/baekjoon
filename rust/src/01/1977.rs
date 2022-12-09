use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.lines().map(|s| s.parse::<i32>().unwrap());

    let (m, n) = (input.next().unwrap(), input.next().unwrap());
    let square_nums: Vec<_> = (1..)
        .skip_while(|i| i * i < m)
        .take_while(|i| i * i <= n)
        .map(|i| i * i)
        .collect();

    if square_nums.len() == 0 {
        println!("-1");
        return;
    }

    println!(
        "{}\n{}",
        square_nums.iter().sum::<i32>(),
        square_nums.iter().min().unwrap()
    );
}
