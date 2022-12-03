use std::io::{stdin, stdout, BufReader, BufWriter, Read, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (BufReader::new(stdin.lock()), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let len = input.next().unwrap() as usize;
    let (mut sum, mut max_count) = (0, 1);
    let mut counts = [0; 8001];

    let mut arr: Vec<_> = input
        .map(|num| {
            sum += num;
            let index = (num + 4000) as usize;

            counts[index] += 1;
            max_count = counts[index].max(max_count);

            num
        })
        .collect();

    arr.sort();

    let (min, max, middle) = (arr[0], arr[len - 1], arr[len / 2]);
    let avg = (sum as f64 / len as f64).round() as i32;

    let mut max_counts: Vec<_> = counts
        .iter()
        .enumerate()
        .filter_map(|(i, &c)| (c == max_count).then(|| i as i32 - 4000))
        .collect();

    let most = if max_counts.len() > 1 {
        max_counts.sort();
        max_counts[1]
    } else {
        max_counts[0]
    };

    for value in [avg, middle, most, max - min] {
        writeln!(stdout, "{value}").unwrap();
    }
}
