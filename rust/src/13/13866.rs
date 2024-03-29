fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let levels = parse_int_vec(&buf);
    let mut min_diff = u32::MAX;

    for i in 1..levels.len() {
        let team1 = levels[0] + levels[i];
        let team2: i32 = levels
            .iter()
            .enumerate()
            .filter_map(|(idx, level)| (idx != 0 && idx != i).then_some(level))
            .sum();

        min_diff = team1.abs_diff(team2).min(min_diff);
    }

    println!("{min_diff}");
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
