use std::fmt::Write;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let buf = io::read_to_string(io::stdin())?;
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let mut memo = vec![Vec::new(); 4999];

    for _ in 0..input() {
        let (query, n) = (input(), input());
        let half = n >> 1;
        let offset = half as i32 + 1;
        let i = half - 1;

        if memo[i].is_empty() {
            let mut max_nums = vec![0; half + 1];
            let mut side_len = n as u32;

            for j in 0..=half {
                let last = if j == 0 { 0 } else { max_nums[j - 1] };

                max_nums[j] = last + if side_len == 1 { 1 } else { (side_len - 1) * 4 };
                side_len = side_len.saturating_sub(2);
            }

            memo[i] = max_nums;
            // println!("{:?} {:?}", &memo[..=2], memo[4998]);
        }

        match query {
            1 => {
                let (x, y) = (input(), input());
                let (x, y) = (y as i32 - offset, x as i32 - offset);
                let num = x.abs().max(y.abs());
                let diff = num as u32 * 2;
                // println!("{y} {x} {num}");
                let left_top = if i as i32 - num == -1 {
                    1
                } else {
                    memo[i][i - num as usize] + 1
                };
                if y == -num {
                    writeln!(output, "{}", left_top + x.abs_diff(-num))?;
                    continue;
                }

                let right_top = left_top + diff;
                if x == num {
                    writeln!(output, "{}", right_top + y.abs_diff(-num))?;
                    continue;
                }

                let right_bottom = right_top + diff;
                if y == num {
                    writeln!(output, "{}", right_bottom + x.abs_diff(num))?;
                    continue;
                }

                let left_bottom = right_bottom + diff;
                writeln!(output, "{}", left_bottom + y.abs_diff(num))?;
                // println!("{left_top} {right_top} {right_bottom} {left_bottom}");
            }
            2 => {
                let z = input() as u32;
                let idx = memo[i].partition_point(|&max_num| z > max_num);
                let num = (half - idx) as i32;
                let diff = num as u32 * 2;
                // println!("idx: {idx}, num: {num}");
                let (x, y);
                let left_top = if idx == 0 { 1 } else { memo[i][idx - 1] + 1 };
                let right_top = left_top + diff;
                if z <= right_top {
                    (y, x) = (-num, -num + z.abs_diff(left_top) as i32);
                    writeln!(output, "{} {}", y + offset, x + offset)?;
                    continue;
                }

                let right_bottom = right_top + diff;
                if z <= right_bottom {
                    (y, x) = (-num + z.abs_diff(right_top) as i32, num);
                    writeln!(output, "{} {}", y + offset, x + offset)?;
                    continue;
                }

                let left_bottom = right_bottom + diff;
                if z <= left_bottom {
                    (y, x) = (num, num - z.abs_diff(right_bottom) as i32);
                    writeln!(output, "{} {}", y + offset, x + offset)?;
                    continue;
                }

                (y, x) = (num - z.abs_diff(left_bottom) as i32, -num);
                writeln!(output, "{} {}", y + offset, x + offset)?;
            }
            _ => unreachable!(),
        }
    }

    print!("{output}");
    Ok(())
}
