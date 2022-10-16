fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let nums: Vec<i32> = buf.split_whitespace().map(|s| s.parse().unwrap()).collect();

    let offset = 45;

    if let [mut hour, mut minute] = nums[..] {
        if minute < offset {
            minute = 60 - (offset - minute);
            hour = (hour + (24 - 1)) % 24;
        } else {
            minute -= offset;
        }

        println!("{hour} {minute}");
    }
}
