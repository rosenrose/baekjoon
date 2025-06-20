use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.trim().split(['+', '=']).map(get_coef_counts);

    let [m1, m2, m3] = [(); 3].map(|_| input.next().unwrap());
    // println!("{m1:?} {m2:?} {m3:?}");
    for x1 in 1..=10 {
        let m1 = m1.map(|m| m * x1);

        for x2 in 1..=10 {
            let m2 = m2.map(|m| m * x2);

            for x3 in 1..=10 {
                let m3 = m3.map(|m| m * x3);

                if m1
                    .iter()
                    .zip(&m2)
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
