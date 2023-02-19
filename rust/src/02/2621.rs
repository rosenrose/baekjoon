use std::collections::HashSet;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let mut counts = [0; 10];
    let mut colors = HashSet::new();
    let mut nums: Vec<_> = (0..5)
        .map(|_| {
            let (color, num) = (input.next().unwrap(), parse_int(input.next().unwrap()));
            colors.insert(color);
            counts[num] += 1;

            num
        })
        .collect();
    nums.sort();

    let diff = nums[0].abs_diff(nums[1]);
    let biggest = nums.last().unwrap();

    let is_sequence = (2..nums.len()).all(|i| nums[i - 1].abs_diff(nums[i]) == diff);
    let get_num_by_count = |count: i32| {
        let (num, _) = counts.iter().enumerate().find(|(_, &c)| c == count)?;
        Some(num)
    };

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
                if let Some(num4) = get_num_by_count(4) {
                    num4 + 800
                } else {
                    0
                }
            }
            3 => {
                if let Some(num3) = get_num_by_count(3) {
                    if let Some(num2) = get_num_by_count(2) {
                        return num3 * 10 + num2 + 700;
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
                if let Some(num3) = get_num_by_count(3) {
                    num3 + 400
                } else {
                    0
                }
            }
            7 => {
                if let Some(num2_small) = get_num_by_count(2) {
                    if let Some((num2_big, _)) = counts
                        .iter()
                        .enumerate()
                        .rfind(|&(n, &c)| n != num2_small && c == 2)
                    {
                        return num2_big * 10 + num2_small + 300;
                    }
                }

                0
            }
            8 => {
                if let Some(num2) = get_num_by_count(2) {
                    num2 + 200
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

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
