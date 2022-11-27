fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    if let [n, m] = parse_int_vec(&buf)[..] {
        read_line(&mut buf);

        let nums = parse_int_vec(&buf);
        let mut count = 0;

        for bit in 1..1 << n {
            let indices: Vec<_> = (0..n).filter(|index| bit & (1 << index) != 0).collect();
            let mut lcm = 1;

            for num in indices.iter().map(|&i| nums[i as usize]) {
                lcm *= num;

                if lcm > m {
                    break;
                }
            }

            let multiple_count = m / lcm;

            count += if indices.len() % 2 == 1 {
                multiple_count
            } else {
                -multiple_count
            };
        }

        println!("{count}");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i64> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
