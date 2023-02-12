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

fn get_coef_counts(s: &str) -> [i32; 3] {
    let mut last_atom = '\0';
    let idx = |ch: char| match ch {
        'C' => 0,
        'H' => 1,
        'O' => 2,
        _ => Default::default(),
    };

    s.chars().fold([0; 3], |mut acc, ch| {
        match ch {
            'C' | 'H' | 'O' => {
                acc[idx(ch)] += 1;
                last_atom = ch;
            }
            num @ '2'..='9' => {
                acc[idx(last_atom)] -= 1;
                acc[idx(last_atom)] += num as i32 - '0' as i32;
            }
            _ => (),
        }

        acc
    })
}
