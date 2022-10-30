fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let nums = buf.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    let default = [1, 1, 2, 2, 2, 8];
    let result = default.iter().zip(nums).map(|(x, y)| x - y);

    for r in result {
        print!("{r} ");
    }
}
