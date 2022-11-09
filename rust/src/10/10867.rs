use std::collections::BTreeSet;

fn main() {
    let mut buf = String::new();
    read_line(&mut buf);
    read_line(&mut buf);

    let nums: BTreeSet<_> = buf
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    for num in nums {
        print!("{num} ");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
