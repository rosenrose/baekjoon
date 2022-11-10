fn main() {
    let mut buf = String::new();

    for _ in 0..3 {
        read_line(&mut buf);

        let (mut count, mut max_count) = (0, 0);
        let mut current = buf.chars().nth(0).unwrap();

        for next in buf.trim().chars() {
            if current == next {
                count += 1;
                continue;
            }

            max_count = count.max(max_count);
            count = 1;
            current = next;
        }

        max_count = count.max(max_count);

        println!("{max_count}");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
