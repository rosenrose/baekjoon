use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.lines().map(|s| s.parse::<i32>().unwrap());

    let class1: Vec<_> = (0..4).map(|_| input.next().unwrap()).collect();
    let class2: Vec<_> = input.collect();
    let mut max_sum = 0;

    for c1 in class1.iter() {
        for c2 in class2.iter() {
            let sum = (class1.iter().sum::<i32>() - c1) + c2;
            max_sum = sum.max(max_sum);
        }
    }

    println!("{max_sum}");
}
