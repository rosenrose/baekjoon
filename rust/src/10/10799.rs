fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut laser_counts = Vec::with_capacity(50_000);
    let mut bar_count = 0;
    let mut last_ch = '\0';

    for ch in buf.trim().chars() {
        match ch {
            '(' => laser_counts.push(0),
            ')' => {
                let last_laser = laser_counts.last_mut().unwrap();

                if last_ch == '(' {
                    *last_laser += 1;
                } else {
                    bar_count += *last_laser + 1;
                }

                let temp = laser_counts.pop().unwrap();

                if let Some(last_laser) = laser_counts.last_mut() {
                    *last_laser += temp;
                } else {
                    laser_counts.push(temp);
                }
            }
            _ => unreachable!(),
        }

        last_ch = ch;
    }

    println!("{bar_count}");
}
