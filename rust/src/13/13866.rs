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
            .filter(|&(idx, _)| idx != 0 && idx != i)
            .map(|(_, level)| level)
            .sum();

        let diff = team1.abs_diff(team2);
        if diff < min_diff {
            min_diff = diff;
        }
    }

    println!("{min_diff}");
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
