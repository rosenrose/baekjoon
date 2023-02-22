use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(str::parse::<i32>);

    let len = input.next().unwrap() as usize;
    let mut sum = 0;
    let mut counts = [0; 8001];

    let mut arr: Vec<_> = input
        .map(|num| {
            sum += num;
            counts[(num + 4000) as usize] += 1;

            num
        })
        .collect();

    arr.sort_unstable();

    let (min, max, middle) = (arr[0], arr[len - 1], arr[len / 2]);
    let avg = (sum as f64 / len as f64).round() as i32;

    let max_count = counts.iter().max().unwrap();
    let mut max_counts: Vec<_> = counts
        .iter()
        .enumerate()
        .filter_map(|(i, c)| (c == max_count).then_some(i as i32 - 4000))
        .collect();

    let most = if max_counts.len() > 1 {
        *max_counts.select_nth_unstable(1).1
    } else {
        max_counts[0]
    };

    for value in [avg, middle, most, max - min] {
        println!("{value}");
    }
}
