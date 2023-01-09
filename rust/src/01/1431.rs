use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut numbers: Vec<_> = buf.lines().skip(1).collect();

    let digit_sum = |s: &str| -> i32 {
        s.chars()
            .filter_map(|c| c.is_numeric().then(|| c as i32 - '0' as i32))
            .sum()
    };

    numbers.sort_by(|a, b| {
        if a.len() == b.len() {
            let (a_sum, b_sum) = (digit_sum(a), digit_sum(b));

            if a_sum == b_sum {
                a.cmp(b)
            } else {
                a_sum.cmp(&b_sum)
            }
        } else {
            a.len().cmp(&b.len())
        }
    });

    println!("{}", numbers.join("\n"));
}
