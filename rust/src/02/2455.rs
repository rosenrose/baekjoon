fn main() {
    let mut buf = String::new();
    const FULL: i32 = 10000;

    let mut max_count = 0;

    let _final_count = (0..4)
        .map(|_| {
            read_line(&mut buf);
            let nums = parse_int_vec(&buf);

            (nums[0], nums[1])
        })
        .fold(0, |current, (off, on)| {
            let next = (current - off + on).min(FULL);

            if next > max_count {
                max_count = next;
            }

            next
        });

    println!("{max_count}");
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
