fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [s1, s2, s3] = parse_int_vec(&buf)[..] else { return };
    let mut counts = [0; 20 + 20 + 40 + 1];
    let mut max_count = 0;

    for i in 1..=s1 {
        for j in 1..=s2 {
            for k in 1..=s3 {
                counts[i + j + k] += 1;
                max_count = counts[i + j + k].max(max_count);
            }
        }
    }
    // println!("{counts:?}");
    let sum = counts
        .iter()
        .enumerate()
        .filter_map(|(sum, &c)| (c == max_count).then_some(sum))
        .min()
        .unwrap();

    println!("{sum}");
}

fn parse_int_vec(buf: &String) -> Vec<usize> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
