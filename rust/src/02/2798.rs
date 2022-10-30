fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let m = parse_int_vec(&buf)[1];
    read_line(&mut buf);

    let arr = parse_int_vec(&buf);
    let len = arr.len();

    let mut min_sum = 0;

    for i in 0..len {
        for j in i + 1..len {
            for k in j + 1..len {
                let sum = arr[i] + arr[j] + arr[k];

                if sum > m {
                    continue;
                }

                if m - sum < m - min_sum {
                    min_sum = sum;
                }
            }
        }
    }

    println!("{min_sum}");
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
