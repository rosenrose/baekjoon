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
        a.len().cmp(&b.len()).then_with(|| {
            let (a_sum, b_sum) = (digit_sum(a), digit_sum(b));

            (a_sum, a).cmp(&(b_sum, b))
        })
    });

    println!("{}", numbers.join("\n"));
}
