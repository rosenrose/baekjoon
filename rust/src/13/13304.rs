use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let (n, k) = (input(), input() as f64);
    let mut students = [[0, 0]; 7];

    for (gender, grade) in (0..n).map(|_| (input(), input())) {
        students[grade][gender] += 1;
    }

    let mut count = (students[1..=2]
        .iter()
        .map(|row| row.iter().sum::<i32>())
        .sum::<i32>() as f64
        / k)
        .ceil() as i32;
    count += (0..=1)
        .map(|gender| {
            ((3..=4).map(|grade| students[grade][gender]).sum::<i32>() as f64 / k).ceil() as i32
                + ((5..=6).map(|grade| students[grade][gender]).sum::<i32>() as f64 / k).ceil()
                    as i32
        })
        .sum::<i32>();

    println!("{count}");
}
