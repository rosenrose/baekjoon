use std::cmp::Ordering;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let nums: Vec<i32> = buf.split_whitespace().map(|s| s.parse().unwrap()).collect();

    match nums[0].cmp(&nums[1]) {
        Ordering::Less => println!("<"),
        Ordering::Equal => println!("=="),
        Ordering::Greater => println!(">"),
    }
}
