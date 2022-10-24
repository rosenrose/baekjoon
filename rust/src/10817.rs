fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut nums: Vec<i32> = buf.split_whitespace().map(|s| s.parse().unwrap()).collect();

    nums.sort();

    println!("{}", nums[1]);
}
