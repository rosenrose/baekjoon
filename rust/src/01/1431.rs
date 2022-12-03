fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();
    let mut numbers = parse_str_vec_lines(&mut buf, n);

    let digit_sum = |s: &String| {
        s.chars()
            .filter_map(|c| c.is_numeric().then(|| c.to_digit(10).unwrap()))
            .sum::<u32>()
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

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_str_vec_lines(buf: &mut String, n: i32) -> Vec<String> {
    (0..n)
        .map(|_| {
            read_line(buf);
            buf.trim().to_string()
        })
        .collect()
}
