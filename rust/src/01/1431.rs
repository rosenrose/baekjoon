use std::io;

const MAX: usize = 50;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let n: usize = input.next().unwrap().parse().unwrap();
    let mut numbers = [""; MAX];

    for (i, num) in input.enumerate() {
        numbers[i] = num;
    }

    let digit_sum = |s: &str| {
        s.as_bytes()
            .iter()
            .filter_map(|ch| matches!(ch, b'0'..=b'9').then_some((ch - b'0') as i32))
            .sum::<i32>()
    };

    numbers[..n].sort_by(|a, b| {
        a.len()
            .cmp(&b.len())
            .then_with(|| digit_sum(a).cmp(&digit_sum(b)))
            .then_with(|| a.cmp(b))
    });

    println!("{}", numbers[..n].join("\n"));
}
