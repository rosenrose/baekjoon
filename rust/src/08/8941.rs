use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let regex = Regex::new(r"[CHON][0-9]*", false);

    for formula in buf.lines().skip(1) {
        let mut cursor = 0;
        let mut sum = 0.0;

        while let Some((start, end)) = regex.find(&formula[cursor..]) {
            let molar = &formula[cursor + start..cursor + end];
            let (atom, count) = molar.split_at(1);
            let count: i32 = count.parse().unwrap_or(1);

            sum += match atom {
                "C" => 12.01,
                "H" => 1.008,
                "O" => 16.00,
                "N" => 14.01,
                _ => Default::default(),
            } * count as f64;

            cursor += end;
        }

        println!("{sum:.3}");
    }
}
