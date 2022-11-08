fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();

    for _ in 0..n {
        read_line(&mut buf);
        read_line(&mut buf);

        let mut stores = parse_int_vec(&buf);
        stores.sort();

        let min = stores[0];
        let max = *stores.last().unwrap();

        let min_distance = (min..=max)
            .map(|i| {
                i.abs_diff(min)
                    + i.abs_diff(max)
                    + (0..stores.len() - 1)
                        .map(|j| stores[j].abs_diff(stores[j + 1]))
                        .sum::<u32>()
            })
            .min()
            .unwrap();

        println!("{min_distance}");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
