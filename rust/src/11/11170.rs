fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let t: i32 = buf.trim().parse().unwrap();

    for _ in 0..t {
        read_line(&mut buf);

        if let [n, m] = parse_int_vec(&buf)[..] {
            let count_sum: i32 = (n..=m)
                .map(|mut i| {
                    if i == 0 {
                        return 1;
                    }

                    let mut count = 0;

                    while i > 0 {
                        if i % 10 == 0 {
                            count += 1;
                        }

                        i /= 10;
                    }

                    count
                })
                .sum();

            println!("{count_sum}");
        }
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
