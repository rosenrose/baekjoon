fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut laser_counts = [0; 50_001];
    let mut bar_count = 0;
    let mut depth = 0;
    let mut last = '\0';

    for ch in buf.trim().chars() {
        match ch {
            '(' => depth += 1,
            ')' => {
                if last == '(' {
                    laser_counts[depth] += 1;
                } else {
                    bar_count += laser_counts[depth] + 1;
                }

                laser_counts[depth - 1] += laser_counts[depth];
                laser_counts[depth] = 0;
                depth -= 1;
            }
            _ => unreachable!(),
        }

        last = ch;
    }

    println!("{bar_count}");
}
