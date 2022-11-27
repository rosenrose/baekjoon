use std::collections::HashMap;
use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut sum = 0;
    let mut counts = HashMap::new();
    let mut max_count = 1;

    let mut arr: Vec<_> = buf
        .lines()
        .skip(1)
        .map(|line| {
            let num: i32 = line.parse().unwrap();
            sum += num;

            counts
                .entry(num)
                .and_modify(|count: &mut i32| {
                    *count += 1;
                    max_count = (*count).max(max_count);
                })
                .or_insert(1);

            num
        })
        .collect();

    arr.sort();

    let min = arr[0];
    let max = arr.last().unwrap();

    let len = arr.len();
    let middle = arr[len / 2];
    let avg = sum as f64 / len as f64;

    let mut max_counts: Vec<_> = counts.iter().filter(|(_, &c)| c == max_count).collect();

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
