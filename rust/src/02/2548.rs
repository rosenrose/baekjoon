fn main() {
    let mut buf = String::new();
    read_line(&mut buf);
    read_line(&mut buf);

    let mut nums = parse_int_vec(&buf);
    nums.sort();

    let len = nums.len();

    if len % 2 == 1 {
        println!("{}", nums[len / 2]);
        return;
    }

    let (a, b) = (nums[len / 2 - 1], nums[len / 2]);
    let diff_a = nums.iter().map(|num| num.abs_diff(a)).sum::<u32>();
    let diff_b = nums.iter().map(|num| num.abs_diff(b)).sum::<u32>();

    println!("{}", if diff_a <= diff_b { a } else { b });
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
