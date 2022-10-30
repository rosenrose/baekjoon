fn main() {
    let mut buf = String::new();

    loop {
        read_line(&mut buf);
        let nums = parse_int_vec(&buf);

        if nums.iter().all(|&n| n == 0) {
            return;
        }

        let (index, &longest) = nums.iter().enumerate().max_by_key(|(_, &n)| n).unwrap();

        let is_right = match index {
            0 => nums[1].pow(2) + nums[2].pow(2) == longest.pow(2),
            1 => nums[0].pow(2) + nums[2].pow(2) == longest.pow(2),
            2 => nums[0].pow(2) + nums[1].pow(2) == longest.pow(2),
            _ => false,
        };

        println!("{}", if is_right { "right" } else { "wrong" });
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
