fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [d, k] = parse_int_vec(&buf)[..] else { return };

    let mut memo = vec![0; d + 1];
    memo[d] = k as i32;

    'outer: for prev in (k + 1) / 2..k {
        memo[d - 1] = prev as i32;

        for i in (1..=d - 2).rev() {
            memo[i] = memo[i + 2] - memo[i + 1];

            if memo[i] < 1 || memo[i] > memo[i + 1] {
                continue 'outer;
            }
        }

        break;
    }
    // println!("{memo:?}");
    println!("{}\n{}", memo[1], memo[2]);
}

fn parse_int_vec(buf: &String) -> Vec<usize> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
