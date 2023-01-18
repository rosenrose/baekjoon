fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [d, k] = parse_int_vec(&buf)[..] else { return };

    let mut cache = vec![0; d + 1];
    cache[d] = k as i32;

    'outer: for prev in (k + 1) / 2..k {
        cache[d - 1] = prev as i32;

        for i in (1..=d - 2).rev() {
            cache[i] = cache[i + 2] - cache[i + 1];

            if cache[i] < 1 || cache[i] > cache[i + 1] {
                continue 'outer;
            }
        }

        break;
    }
    // println!("{cache:?}");
    println!("{}\n{}", cache[1], cache[2]);
}

fn parse_int_vec(buf: &String) -> Vec<usize> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
