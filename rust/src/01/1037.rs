fn main() {
    let mut buf = String::new();
    read_line(&mut buf);
    read_line(&mut buf);

    let mut nums = parse_int_vec(&buf);
    nums.sort();

    println!("{}", nums[0] * nums.last().unwrap());
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
