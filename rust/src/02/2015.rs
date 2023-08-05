use std::collections::HashMap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [n, k] = [(); 2].map(|_| input.next().unwrap());
    let mut sum_counts = HashMap::with_capacity(n as usize >> 1);
    sum_counts.insert(0, 1_i64);

    let mut sum = 0;
    let mut count = 0;

    for num in input {
        sum += num;
        count += sum_counts.get(&(sum - k)).unwrap_or(&0);
        sum_counts.entry(sum).and_modify(|c| *c += 1).or_insert(1);
    }

    println!("{count}");
}
