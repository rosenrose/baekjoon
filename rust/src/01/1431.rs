use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut numbers: Vec<_> = buf.lines().skip(1).collect();

    let digit_sum = |s: &str| {
        s.chars()
            .filter_map(|c| c.is_numeric().then_some(c as i32 - '0' as i32))
            .sum::<i32>()
    };

    numbers.sort_by(|a, b| {
        a.len()
            .cmp(&b.len())
            .then_with(|| digit_sum(a).cmp(&digit_sum(b)))
            .then_with(|| a.cmp(b))
    });

    println!("{}", numbers.join("\n"));
}
