use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mass = |ch: char| match ch {
        'C' => 12.01,
        'H' => 1.008,
        'O' => 16.00,
        'N' => 14.01,
        _ => Default::default(),
    };

    for formula in buf.lines().skip(1) {
        let mut sum = 0.0;
        let mut last_ch = '\0';
        let mut last_num = 0;

        for ch in formula.chars() {
            match ch {
                'C' | 'H' | 'O' | 'N' => {
                    if last_num > 0 {
                        sum += mass(last_ch) * (last_num - 1) as f64;
                        last_num = 0;
                    }

                    sum += mass(ch);
                    last_ch = ch;
                }
                '0'..='9' => {
                    let num = ch as i32 - '0' as i32;

                    if last_num > 0 {
                        last_num *= 10;
                    }

                    last_num += num;
                }
                _ => (),
            }
        }

        if last_num > 0 {
            sum += mass(last_ch) * (last_num - 1) as f64;
        }

        println!("{sum:.3}");
    }
}
