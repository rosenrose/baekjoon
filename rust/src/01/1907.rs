fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let input: Vec<_> = buf.trim().split(['+', '=']).map(get_coef_counts).collect();
    let [m1, m2, m3] = input[..] else { return };
    // println!("{m1:?} {m2:?} {m3:?}");
    for x1 in 1..=10 {
        let m1: Vec<_> = m1.iter().map(|m| m * x1).collect();

        for x2 in 1..=10 {
            let m2: Vec<_> = m2.iter().map(|m| m * x2).collect();

            for x3 in 1..=10 {
                let m3: Vec<_> = m3.iter().map(|m| m * x3).collect();

                if m1
                    .iter()
                    .zip(m2.iter())
                    .map(|(m1, m2)| m1 + m2)
                    .eq(m3.into_iter())
                {
                    println!("{x1} {x2} {x3}");
                    return;
                }
            }
        }
    }
}

fn get_coef_counts(input: &str) -> [i32; 3] {
    let regex = Regex::new(r"[CHO][2-9]?", false);
    let mut cursor = 0;
    let mut counts = [0; 3];

    while let Some((start, end)) = regex.find(&input[cursor..]) {
        let molar = &input[cursor + start..cursor + end];
        let (atom, count) = molar.split_at(1);
        let count: i32 = count.parse().unwrap_or(1);

        match atom {
            "C" => counts[0] += count,
            "H" => counts[1] += count,
            "O" => counts[2] += count,
            _ => unreachable!(),
        }

        cursor += end;
    }

    counts
}
