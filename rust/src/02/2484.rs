use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let max_prize = (0..input.next().unwrap())
        .map(|_| {
            let nums: Vec<_> = input.by_ref().take(4).collect();
            let mut counts = [0; 7];

            for &num in &nums {
                counts[num] += 1;
            }

            if let Some(a) = counts.iter().position(|&c| c == 4) {
                return 50000 + a * 5000;
            }

            if let Some(a) = counts.iter().position(|&c| c == 3) {
                return 10000 + a * 1000;
            }

            if let Some(a) = counts.iter().position(|&c| c == 2) {
                let b = counts.iter().rposition(|&c| c == 2).unwrap();

                return if a != b {
                    2000 + a * 500 + b * 500
                } else {
                    1000 + a * 100
                };
            }

            nums.iter().max().unwrap() * 100
        })
        .max()
        .unwrap();

    println!("{max_prize}");
}
