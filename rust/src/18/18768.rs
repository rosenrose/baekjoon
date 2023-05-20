use std::cmp::Reverse;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    for _ in 0..input() {
        let (n, k) = (input() as usize, input() as usize);
        let mut scores = vec![(0, 0); n];

        for i in 0..n {
            scores[i].0 = input();
        }
        for i in 0..n {
            scores[i].1 = input();
        }

        let min_count = ((n - k) + 1) / 2;
        let max_count = n - min_count;

        scores.sort_unstable_by_key(|&(atk, def)| Reverse(atk.abs_diff(def)));
        // println!("{scores:?}");
        let (mut atk_count, mut def_count) = (0, 0);
        let mut max_sum = 0;

        for (atk, def) in scores {
            if atk_count == max_count {
                max_sum += def as i64;
                continue;
            }
            if def_count == max_count {
                max_sum += atk as i64;
                continue;
            }

            if atk > def {
                max_sum += atk as i64;
                atk_count += 1;
            } else {
                max_sum += def as i64;
                def_count += 1;
            }
        }

        println!("{max_sum}");
    }
}
