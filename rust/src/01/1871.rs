use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    for input in buf.lines().skip(1) {
        let (first, second) = input.split_at(3);

        let first_value: i32 = first
            .chars()
            .rev()
            .enumerate()
            .map(|(i, c)| (c as u8 - 'A' as u8) as i32 * 26_i32.pow(i as u32))
            .sum();

        let second_value: i32 = second[1..].parse().unwrap();

        println!(
            "{}",
            if first_value.abs_diff(second_value) <= 100 {
                "nice"
            } else {
                "not nice"
            }
        );
    }
}
