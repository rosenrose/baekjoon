use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for input in buf.lines().skip(1) {
        let (first, second) = input.split_at(3);

        let first_value: i32 = first
            .chars()
            .rev()
            .enumerate()
            .map(|(i, ch)| (ch as u8 - b'A') as i32 * 26_i32.pow(i as u32))
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
