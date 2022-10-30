fn main() {
    let mut buf = String::new();

    let mut max = -1;
    let (mut x, mut y) = (0, 0);

    for i in 0..9 {
        read_line(&mut buf);
        let nums = buf.split_whitespace().map(|s| s.parse::<i32>().unwrap());

        for (j, num) in nums.enumerate() {
            if num > max {
                (max, x, y) = (num, i + 1, j + 1);
            }
        }
    }

    println!("{max}\n{x} {y}");
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
