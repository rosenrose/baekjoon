fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();

    for _ in 0..n {
        read_line(&mut buf);

        if let [x, y] = parse_int_vec(&buf)[..] {
            let distance = y - x;

            let mut step = 0;
            let mut max_distance = 0;

            while max_distance < distance {
                step += 1;

                max_distance += if step % 2 == 0 {
                    step / 2
                } else {
                    (step + 1) / 2
                };
            }

            println!("{}", step);
        }
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i64> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

/*
1 - 1
2 - 1 1
3 - 1 2 1
4 - 1 2 2 1
5 - 1 2 3 2 1
6 - 1 2 3 3 2 1
7 - 1 2 3 4 3 2 1
8 - 1 2 3 4 4 3 2 1
*/
