fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [mut n, b] = parse_int_vec(&buf)[..] {
        let mut b_nums = Vec::new();

        while n != 0 {
            b_nums.push(n % b);
            n /= b;
        }

        b_nums.iter().rev().for_each(|&b_num| {
            match b_num {
                0..=9 => print!("{b_num}"),
                10.. => print!("{}", ('A' as u8 + (b_num as u8 - 10)) as char),
                _ => (),
            };
        });
    }
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
