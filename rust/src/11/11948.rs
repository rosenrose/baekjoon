use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(str::parse::<usize>);

    let class1: Vec<_> = input.by_ref().take(4).collect();
    let class2 = input;
    let mut max_sum = 0;

    for c2 in class2 {
        for c1 in class1.iter() {
            let sum = (class1.iter().sum::<usize>() - c1) + c2;
            max_sum = sum.max(max_sum);
        }
    }

    println!("{max_sum}");
}
