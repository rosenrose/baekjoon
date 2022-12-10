use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    for input in buf.lines() {
        let (mut count, mut max_count) = (0, 0);
        let mut current = input.chars().nth(0).unwrap();

        for next in input.chars() {
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
