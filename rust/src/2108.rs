use std::collections::HashMap;
use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();

    let mut arr: Vec<i32> = (0..n)
        .map(|_| {
            buf.clear();
            stdin.read_line(&mut buf).unwrap();

            buf.trim().parse::<i32>().unwrap()
        })
        .collect();

    arr.sort();

    let len = arr.len();
    let middle = arr[len / 2];

    const N: i32 = 4000;
    let mut sum = 0;
    let (mut min, mut max) = (N, -N);

    let mut counts = HashMap::new();
    let mut max_count = 0;

    for &num in arr.iter() {
        sum += num;

        if num < min {
            min = num;
        }
        if num > max {
            max = num;
        }

        counts
            .entry(num)
            .and_modify(|c| {
                let count = *c + 1;
                if count > max_count {
                    max_count = count;
                }

                count
            })
            .or_insert(1);
    }

    let avg = sum as f64 / len as f64;

    let mut max_counts: Vec<(&i32, &i32)> =
        counts.iter().filter(|(_, &c)| c == max_count).collect();

    let (most, _) = if max_counts.len() > 1 {
        max_counts.sort_by_key(|(&num, _)| num);
        max_counts[1]
    } else {
        max_counts[0]
    };

    for value in [avg.round() as i32, middle, *most, max - min] {
        writeln!(stdout, "{value}").unwrap();
    }
}
