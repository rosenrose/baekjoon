fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut nums: Vec<i32> = buf.split_whitespace().flat_map(str::parse).collect();

    println!("{}", nums.select_nth_unstable(1).1);
}
