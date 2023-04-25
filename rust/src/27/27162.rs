use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let choices = input.next().unwrap();
    let mut counts = [0; 7];
    let nums: Vec<_> = input
        .map(|s| {
            let num: usize = s.parse().unwrap();
            counts[num] += 1;

            num
        })
        .collect();

    let get_num_by_count = |count: i32| counts.iter().position(|&c| c == count);

    let max_score = choices
        .char_indices()
        .filter_map(|(i, ch)| {
            (ch == 'Y').then(|| match i + 1 {
                n @ 1..=6 => (nums.iter().filter(|&&num| num == n).count() + 2) * n,
                7 => {
                    if let Some(a) = get_num_by_count(3) {
                        return a * 4;
                    }
                    if let Some(a) = get_num_by_count(2) {
                        return a * 4;
                    }

                    0
                }
                8 => {
                    if let Some(a) = get_num_by_count(3) {
                        return (a * 3) + (if a == 6 { 5 } else { 6 } * 2);
                    }

                    if let Some(a) = get_num_by_count(2) {
                        let b = get_num_by_count(1).unwrap();

                        return (a.max(b) * 3) + (a.min(b) * 2);
                    }

                    0
                }
                9 => {
                    if counts.iter().filter(|&&c| c == 1).count() == 3 && counts[6] == 0 {
                        30
                    } else {
                        0
                    }
                }
                10 => {
                    if counts.iter().filter(|&&c| c == 1).count() == 3 && counts[1] == 0 {
                        30
                    } else {
                        0
                    }
                }
                11 => {
                    if counts.iter().find(|&&c| c == 3).is_some() {
                        50
                    } else {
                        0
                    }
                }
                12 => nums.iter().sum::<usize>() + (6 * 2),
                _ => Default::default(),
            })
        })
        .max()
        .unwrap();

    println!("{max_score}");
}
