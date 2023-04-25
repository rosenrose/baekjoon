use std::collections::HashSet;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let mut counts = [0; 10];
    let mut colors = HashSet::new();
    let mut nums: Vec<_> = (0..5)
        .map(|_| {
            let (color, num) = (
                input.next().unwrap(),
                input.next().unwrap().parse::<usize>().unwrap(),
            );
            colors.insert(color);
            counts[num] += 1;

            num
        })
        .collect();
    nums.sort();

    let diff = nums[0].abs_diff(nums[1]);
    let is_sequence = (2..nums.len()).all(|i| nums[i - 1].abs_diff(nums[i]) == diff);
    let biggest = nums.last().unwrap();

    let get_num_by_count = |count: i32| counts.iter().position(|&c| c == count);

    let max_score = (1..=9)
        .map(|rule| match rule {
            1 => {
                if colors.len() == 1 && is_sequence {
                    biggest + 900
                } else {
                    0
                }
            }
            2 => {
                if let Some(a) = get_num_by_count(4) {
                    a + 800
                } else {
                    0
                }
            }
            3 => {
                if let Some(a) = get_num_by_count(3) {
                    if let Some(b) = get_num_by_count(2) {
                        return a * 10 + b + 700;
                    }
                }

                0
            }
            4 => {
                if colors.len() == 1 {
                    biggest + 600
                } else {
                    0
                }
            }
            5 => {
                if is_sequence {
                    biggest + 500
                } else {
                    0
                }
            }
            6 => {
                if let Some(a) = get_num_by_count(3) {
                    a + 400
                } else {
                    0
                }
            }
            7 => {
                if let Some(a) = get_num_by_count(2) {
                    let b = counts.iter().rposition(|&c| c == 2).unwrap();

                    return if a != b { b * 10 + a + 300 } else { 0 };
                }

                0
            }
            8 => {
                if let Some(a) = get_num_by_count(2) {
                    a + 200
                } else {
                    0
                }
            }
            9 => biggest + 100,
            _ => Default::default(),
        })
        .max()
        .unwrap();

    println!("{max_score}");
}
