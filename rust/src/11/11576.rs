fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    if let [a, b] = parse_int_vec(&buf)[..] {
        read_line(&mut buf);
        read_line(&mut buf);
        let mut b_nums = Vec::new();

        let mut a_num: i128 = buf
            .split_whitespace()
            .rev()
            .enumerate()
            .map(|(i, s)| s.parse::<i128>().unwrap() * a.pow(i as u32))
            .sum();

        while a_num != 0 {
            b_nums.push(a_num % b);
            a_num /= b;
        }

        for b_num in b_nums.iter().rev() {
            print!("{b_num} ");
        }
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i128> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
